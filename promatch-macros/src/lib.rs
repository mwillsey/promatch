extern crate proc_macro;

use std::collections::HashMap;

use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Expr, Ident, Lit, Pat, PatType, Result, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    token::Brace,
};

macro_rules! err {
    ($span:expr, $($rest:tt)*) => { Err(syn::Error::new($span, format!($($rest)*))) };
}

fn ident_to_pat(ident: Ident) -> Pat {
    Pat::Ident(syn::PatIdent {
        attrs: Vec::new(),
        by_ref: None,
        mutability: None,
        ident: ident.clone(),
        subpat: None,
    })
}

#[derive(Parse)]
struct MyMatch {
    ctx: Expr,
    _match_token: Token![match],
    #[call(Expr::parse_without_eager_brace)]
    expr: Expr,
    #[brace]
    _brace_token: Brace,
    #[inside(_brace_token)]
    #[call(parse_arms)]
    arms: Vec<MyArm>,
}

#[derive(Parse)]
struct MyArm {
    #[call(parse_pat)]
    pat: Pat,
    _arrow: Token![=>],
    body: Expr,
    _comma: Option<Token![,]>,
}

fn parse_arms(input: ParseStream) -> Result<Vec<MyArm>> {
    let mut arms = Vec::new();
    while !input.is_empty() {
        arms.push(input.call(Parse::parse)?);
    }
    Ok(arms)
}

fn parse_pat(input: ParseStream) -> Result<Pat> {
    let mut pat = Pat::parse_single(input)?;
    if input.peek(Token![:]) {
        let colon_token: Token![:] = input.parse()?;
        let ty: Type = input.parse()?;
        pat = Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(pat),
            colon_token,
            ty: Box::new(ty),
        });
    }
    if input.peek(Token![=>]) {
        Ok(pat)
    } else {
        err!(pat.span(), "Expected => after pattern")
    }
}

#[derive(Default)]
struct ArmCompiler {
    bound: HashMap<String, Ident>,
    instructions: Vec<Instruction>,
    n: usize,
}

enum Instruction {
    Bind(Ident, Pat),
    CheckEq(Ident, Ident),
    CheckLit(Ident, Lit),
}

impl ArmCompiler {
    fn tokens(ctx: &Ident, argument: &Expr, arm: &MyArm) -> TokenStream {
        let mut x = ArmCompiler::default();

        let top_ident = match x.go(&arm.pat) {
            Ok(ident) => ident,
            Err(e) => return e.into_compile_error().into(),
        };

        let mut output = arm.body.to_token_stream();

        for inst in &x.instructions {
            output = match inst {
                Instruction::Bind(ident, pat) => quote! {
                    #ctx.unapply(#ident, |#[allow(unused_variables)] #ctx, #pat| { #output })
                },
                Instruction::CheckEq(ident, ident2) => quote! { if #ident == #ident2 { #output } },
                Instruction::CheckLit(ident, lit) => quote! { if #ident == #lit { #output } },
            }
            // output = quote! {
            //     #ctx.unapply(#ident, |#[allow(unused_variables)] #ctx, #pat| { #output })
            // };
        }

        quote! { let #top_ident = #argument; #output }.into()
    }

    fn fresh(&mut self) -> usize {
        let n = self.n;
        self.n += 1;
        n
    }

    fn go(&mut self, pattern: &Pat) -> Result<Ident> {
        match pattern {
            Pat::Ident(i) => {
                assert!(i.subpat.is_none(), "not supported yet");
                let s = i.ident.to_string();
                if let Some(ident) = self.bound.get(&s).cloned() {
                    let ident2 = format_ident!("v{}_{}", self.fresh(), ident);
                    self.instructions
                        .push(Instruction::CheckEq(ident.clone(), ident2.clone()));
                    return Ok(ident2);
                } else {
                    self.bound.insert(s.clone(), i.ident.clone());
                    Ok(i.ident.clone())
                }
            }
            Pat::Lit(p) => {
                let ident = format_ident!("lit{}", self.fresh());
                self.instructions
                    .push(Instruction::CheckLit(ident.clone(), p.lit.clone()));
                Ok(ident)
            }
            Pat::TupleStruct(p) => {
                let mut p = p.clone();
                for elem in &mut p.elems {
                    *elem = ident_to_pat(self.go(elem)?);
                }
                let s = p.path.segments.last().unwrap().ident.to_string();
                let s = format!("v{}_{}", self.fresh(), heck::AsSnakeCase(s));
                let ident = Ident::new(&s, p.span());
                self.instructions
                    .push(Instruction::Bind(ident.clone(), Pat::TupleStruct(p)));
                Ok(ident)
            }
            Pat::Slice(p) => {
                let mut p = p.clone();
                for elem in &mut p.elems {
                    *elem = ident_to_pat(self.go(elem)?);
                }
                let s = format!("v{}_slice", self.fresh());
                let ident = Ident::new(&s, p.span());
                self.instructions
                    .push(Instruction::Bind(ident.clone(), Pat::Slice(p)));
                Ok(ident)
            }
            _ => {
                let dbg_message = format!("{:?}", pattern);
                let just_the_type = dbg_message.split_whitespace().next().unwrap();
                err!(pattern.span(), "Unsupported pattern: {:?}", just_the_type)
            }
        }
    }
}

#[proc_macro]
pub fn promatch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as MyMatch);

    let e = &input.expr;
    let ctx = &input.ctx;
    let ctx_ident = Ident::new("ctx", ctx.span());
    let arm_to_token = |arm| ArmCompiler::tokens(&ctx_ident, &e, arm);
    let tokens = input.arms.iter().map(arm_to_token);
    quote! { let #ctx_ident = #ctx; #(#tokens)* }.into()
}

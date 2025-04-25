use std::collections::BTreeSet;

use crate::{Combine, Search};

pub struct Nil;
macro_rules! impl_nil {
    ($t:ident, $coll:ty) => {
        impl<Ctx, $t> Search<$coll, Nil> for Ctx {
            fn unapply<C>(&self, a: $coll, f: impl Fn(Nil) -> C) -> C
            where
                Self: Combine<C>,
            {
                self.then(a.is_empty(), || f(Nil))
            }
        }
    };
}

impl_nil!(T, Vec<T>);
impl_nil!(T, std::collections::BTreeSet<T>);

pub struct Cons<H, T>(pub H, pub T);

impl<Ctx, T: Clone> Search<Vec<T>, Cons<T, Vec<T>>> for Ctx {
    fn unapply<C>(&self, a: Vec<T>, f: impl Fn(Cons<T, Vec<T>>) -> C) -> C
    where
        Self: Combine<C>,
    {
        let iter = (0..a.len()).map(|i| {
            let head = a[i].clone();
            let tail = a[0..i].iter().chain(a[i + 1..].iter()).cloned().collect();
            f(Cons(head.clone(), tail))
        });
        self.reduce(iter)
    }
}

impl<Ctx, T: Clone> Search<BTreeSet<T>, Cons<T, BTreeSet<T>>> for Ctx {
    fn unapply<C>(&self, a: BTreeSet<T>, f: impl Fn(Cons<T, BTreeSet<T>>) -> C) -> C
    where
        Self: Combine<C>,
    {
        todo!()
        // let iter = (0..a.len()).map(|i| {
        //     let head = a[i].clone();
        //     let tail = a[0..i].iter().chain(a[i + 1..].iter()).cloned().collect();
        //     f(Cons(head.clone(), tail))
        // });
        // self.reduce(iter)
    }
}

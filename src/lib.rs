#[cfg(feature = "macros")]
pub use promatch_macros::*;

pub trait Search<A, B> {
    fn unapply(&self, a: A, f: impl FnMut(&Self, B));
}

trait Context {}

impl Context for () {}

// impl<Ctx: Context, A, B: TryFrom<A>> Search<A, B> for Ctx {
//     fn unapply(&self, a: A, mut f: impl FnMut(&Self, B)) {
//         if let Ok(b) = B::try_from(a) {
//             f(self, b);
//         }
//     }
// }

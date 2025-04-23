#[cfg(feature = "macros")]
pub use promatch_macros::*;

pub trait Search<A, B> {
    fn unapply(&self, a: A, f: impl FnMut(&Self, B));
}

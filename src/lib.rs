pub mod collections;
pub use collections::*;

#[cfg(feature = "macros")]
pub use promatch_macros::*;

pub trait Search<A, B> {
    fn unapply<C>(&self, a: A, f: impl Fn(B) -> C) -> C
    where
        Self: Combine<C>;
}

pub trait Combine<C> {
    fn zero(&self) -> C;
    fn plus(&self, a: C, b: C) -> C;
    fn reduce(&self, iter: impl IntoIterator<Item = C>) -> C {
        iter.into_iter()
            .reduce(|a, b| self.plus(a, b))
            .unwrap_or_else(|| self.zero())
    }
    fn then(&self, bool: bool, then: impl FnOnce() -> C) -> C {
        if bool { then() } else { self.zero() }
    }
}

impl<Ctx> Combine<()> for Ctx {
    fn zero(&self) -> () {
        ()
    }

    fn plus(&self, _a: (), _b: ()) -> () {
        ()
    }
}

impl<Ctx, T> Combine<Vec<T>> for Ctx {
    fn zero(&self) -> Vec<T> {
        vec![]
    }

    fn plus(&self, a: Vec<T>, b: Vec<T>) -> Vec<T> {
        let mut result = a;
        result.extend(b);
        result
    }
}

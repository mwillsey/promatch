#[cfg(feature = "macros")]
pub use promatch_macros::*;

pub trait Search<A, B> {
    fn unapply(&self, a: A, f: impl FnMut(&Self, B));
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Sum(u32, u32);

    struct Double(u32);

    struct Ctx;

    impl Search<u32, Sum> for Ctx {
        fn unapply(&self, a: u32, mut f: impl FnMut(&Self, Sum)) {
            (0..=a).for_each(|x| f(self, Sum(x, a - x)));
        }
    }

    impl Search<u32, Double> for Ctx {
        fn unapply(&self, a: u32, mut f: impl FnMut(&Self, Double)) {
            if a % 2 == 0 {
                f(self, Double(a / 2));
            }
        }
    }

    #[test]
    fn test() {
        let ctx = Ctx;
        let mut results = vec![];
        my_match!(ctx match 8 {
            Sum(a, Double(a)) => {
                results.push(a);
            }
        });

        assert_eq!(results, [2]);
    }
}

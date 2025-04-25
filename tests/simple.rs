use promatch::*;

struct Sum(u32, u32);

struct Double(u32);

struct Ctx;

impl Search<u32, Sum> for Ctx {
    fn unapply<C>(&self, a: u32, f: impl Fn(Sum) -> C) -> C
    where
        Self: Combine<C>,
    {
        let iter = (0..=a).map(|x| f(Sum(x, a - x)));
        self.reduce(iter)
    }
}

impl Search<u32, Double> for Ctx {
    fn unapply<C>(&self, a: u32, f: impl Fn(Double) -> C) -> C
    where
        Self: Combine<C>,
    {
        self.then(a % 2 == 0, || f(Double(a / 2)))
    }
}

#[test]
fn test() {
    let ctx = Ctx;
    let results = promatch!(ctx match 6 {
        Sum(a, Double(a)) => {
            vec![a]
        }
    });

    assert_eq!(results, [2]);
}

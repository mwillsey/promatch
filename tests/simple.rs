use promatch::*;

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

// impl<T: Clone, const N: usize> Search<Vec<T>, [T; N]> for Ctx {
//     fn unapply(&self, a: Vec<T>, mut f: impl FnMut(&Self, [T; N])) {
//         if let Ok(a) = <[T; N]>::try_from(a.clone()) {
//             f(self, a);
//         }
//     }
// }

#[test]
fn test() {
    let ctx = Ctx;
    let mut results = vec![];
    promatch!(ctx match 6 {
        Sum(a, Double(a)) => {
            results.push(a);
        }
    });

    // my_match!(ctx match vec![1,2,3] {
    //     [a,b,c]: [_; 3] => {
    //         results.push(a);
    //     }
    // });

    assert_eq!(results, [2]);
}

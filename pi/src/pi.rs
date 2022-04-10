use crate::utils::Params as Params;
use crate::utils::{Term as Term, MyTerm};

use rug::{Rational, Float};

pub type Pi = Float;

pub trait MyPi {
    fn new(n: u32, prec: u32) -> Self;
}

impl MyPi for Pi {
    fn new(n: u32, prec: u32) -> Self {
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let mut pi = Float::new(prec) + Pi::get_term_sum(n, &p, prec);
	pi *= 24;
	pi += 3 * Float::with_val(128, 3).sqrt() / 4;
	pi
    }
}

trait PrivPi {
    fn get_term_sum(n: u32, p: &Params, prec: u32) -> Rational;
}

impl PrivPi for Pi {
    fn get_term_sum(n: u32, p: &Params, prec: u32) -> Rational {
        let mut sum = Rational::new();
        for n in 0..=n {
            sum += <Term as MyTerm>::new(n, &p, prec);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::calc_expected;
    
    #[test]
    fn test_term_sum() {
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let sum = Pi::get_term_sum(5, &p, 128);
	let mut expected = Rational::new();
	expected += calc_expected(2, 3, (2, 3));
	expected += calc_expected(-1, 5, (2, 5));
	expected += calc_expected(-1, 28, (2, 7));
	expected += calc_expected(-1, 72, (2, 9));
	expected += calc_expected(-5, 704, (2, 11));
	expected += calc_expected(-7, 1664, (2, 13));
	println!("{:?}\n{:?}", Float::with_val(128, &sum), Float::with_val(128, &expected));
	assert_eq!(sum, expected);
    }
}

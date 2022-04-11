use rayon::prelude::*;
use rug::{Float, Rational};

use crate::utils::Params;
use crate::utils::{MyTerm, Term};

pub type Pi = Float;

pub trait MyPi {
    fn new(n: u32, prec: u32, threaded: bool) -> Self;
}

impl MyPi for Pi {
    fn new(n: u32, prec: u32, threaded: bool) -> Self {
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let termsum = if threaded {
            Pi::get_term_sum_threaded(n, &p, prec)
        } else {
            Pi::get_term_sum(n, &p, prec)
        };
        let mut pi = Float::new(prec) + termsum;
        pi *= 24;
        pi += 3 * Float::with_val(128, 3).sqrt() / 4;
        pi
    }
}

trait PrivPi {
    fn get_term_sum(limit: u32, p: &Params, prec: u32) -> Rational;
    fn get_term_sum_threaded(limit: u32, p: &Params, prec: u32) -> Rational;
}

impl PrivPi for Pi {
    fn get_term_sum(limit: u32, p: &Params, prec: u32) -> Rational {
        let mut sum = Rational::new();
        for n in 0..limit {
            sum += <Term as MyTerm>::new(n, &p, prec);
        }
        sum
    }

    fn get_term_sum_threaded(limit: u32, p: &Params, prec: u32) -> Rational {
        let mut sum = Vec::new();
        // sum.par_extend(0..limit);
        sum.par_extend(
            (0..limit)
                .into_par_iter()
                .map(|n| <Term as MyTerm>::new(n, &p, prec)),
        );
        sum.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::calc_expected;

    #[test]
    fn test_term_sum() {
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let sum = Pi::get_term_sum(6, &p, 128);
        let mut expected = Rational::new();
        expected += calc_expected(2, 3, (2, 3));
        expected += calc_expected(-1, 5, (2, 5));
        expected += calc_expected(-1, 28, (2, 7));
        expected += calc_expected(-1, 72, (2, 9));
        expected += calc_expected(-5, 704, (2, 11));
        expected += calc_expected(-7, 1664, (2, 13));
        assert_eq!(sum, expected);
    }

    #[test]
    fn test_pi_n100() {
        let n = 100;
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let reg_sum = Pi::get_term_sum(n, &p, 128).to_f64();
        let par_sum = Pi::get_term_sum_threaded(n, &p, 128).to_f64();
        assert_eq!(reg_sum, par_sum);
    }

    #[test]
    fn test_pi_n1000() {
        let n = 1000;
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let reg_sum = Pi::get_term_sum(n, &p, 128).to_f64();
        let par_sum = Pi::get_term_sum_threaded(n, &p, 128).to_f64();
        assert_eq!(reg_sum, par_sum);
    }
}

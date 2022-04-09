use rug::{Assign, Complete, Float, Integer, Rational};
use rug::ops::Pow;

pub type Params = SParams;
pub type Term = STerm;

pub struct SParams {
    alpha: Rational,
    x: Rational,
    limit: Rational,
}

impl SParams {
    pub fn new(alpha: (u32, u32), x: (u32, u32), limit: (u32, u32)) -> Params {
        let alpha = Rational::from(alpha);
        let x = Rational::from(x);
        let limit = Rational::from(limit);
        let p = Params { alpha, x, limit };
        p
    }
}

#[derive(Default, Debug)]
pub struct STerm {
    val: Rational,
}

impl Term {
    pub fn new(n: u32, p: &Params) -> Term {
        let mut power: Rational = Default::default();
        let coefficient = Term::get_coefficient(n, &p);
        power = Rational::from((n, 1));

        // simplify

        Term { val: coefficient }
    }

    fn get_coefficient(n: u32, p: &Params) -> Rational {
        let mut num: Rational = Default::default();
        let mut denom: Integer = Integer::new();

        num += 1;
        for i in 0..n {
            num *= (&p.alpha - i).complete();
        }

        Integer::factorial(n).complete_into(&mut denom);

        return num / denom;
    }

    fn get_power(n: u32, p: &Params) -> Rational {
        Rational::from((n, 1)) + &p.x
    }

    fn integrate(coefficient: Rational, power: Rational) -> (Rational, Rational) {
        let powplus1 = power + 1;
        (coefficient / &powplus1, powplus1)
    }

    pub fn print(self) {
        println!("{:?}", self);
    }
}

trait MyPow {
    fn pow(self, e: Rational) -> Rational;
}

impl MyPow for Rational {
    fn pow(self, e: Rational) -> Rational{
	let mut ret = Rational::new();
	ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coefficient() {
        let p = Params::new((1, 2), (1, 2));
        let ce = Term::get_coefficient(5, &p);
        assert_eq!(ce, Rational::from((7, 256)));
    }

    #[test]
    fn test_power() {
        let p = Params::new((1, 2), (1, 2));
        let pow = Term::get_power(5, &p);
        assert_eq!(pow, Rational::from((11, 2)));
    }

    fn test_integrate() {
        let p = Params::new((1, 2), (1, 2), (1, 4));
        let mut term: Term = Default::default();
        let mut ce = Term::get_coefficient(5, &p);
        let mut pow = Term::get_power(5, &p);
        (ce, pow) = Term::integrate(ce, pow);

	let calc = ce * &p.limit;
	let expected = Rational::from((7,
				       Integer::pow(Integer::from(2), 13)) * 1664);
	assert_eq!(calc, expected);
    }
}

use rug::ops::Pow;
use rug::{Complete, Float, Integer, Rational};

pub type Params = SParams;
pub type Term = Rational;

pub struct SParams {
    // (1+x) ^ alpha
    alpha: Rational,
    x: Rational,
    limit: Rational,
}

impl SParams {
    pub fn new(alpha: (i32, i32), x: (i32, i32), limit: (i32, i32)) -> Params {
        let alpha = Rational::from(alpha);
        let x = Rational::from(x);
        let limit = Rational::from(limit);
        let p = Params { alpha, x, limit };
        p
    }
}

trait Myrat {
    fn pow(&self, e: Rational, prec: u32) -> Float;
}

impl Myrat for Rational {
    fn pow(&self, e: Rational, prec: u32) -> Float {
        let b = Float::with_val(prec, self.numer().to_f64() / self.denom().to_f64());
        let e = Float::with_val(prec, e.numer().to_f64() / e.denom().to_f64());
        b.pow(e)
    }
}

pub trait MyTerm {
    fn new(n: u32, p: &Params, prec: u32) -> Term;
    fn print(self);
    fn get_coefficient(n: u32, p: &Params) -> Rational;
    fn get_power(n: u32) -> Rational;
    fn integrate(coefficient: Rational, power: Rational) -> (Rational, Rational);
}

impl MyTerm for Term {
    fn new(n: u32, p: &Params, prec: u32) -> Term {
        // coefficient
        let ce = Term::get_coefficient(n, &p);
        let pow = Term::get_power(n);
        let (ce, pow) = Term::integrate(ce, pow);
	ce * Myrat::pow(&p.limit, pow, prec).to_rational().unwrap()
    }

    fn get_coefficient(n: u32, p: &Params) -> Rational {
        let mut num = Rational::from(1);
        let mut denom: Integer = Integer::new();

	// calc numerator
        for i in 0..n {
            num *= (&p.alpha - i).complete();
        }
	
	// multiply by x's coefficient
        for _ in 0..n {
            num *= &p.x;
        }
	
        Integer::factorial(n).complete_into(&mut denom);

        return num / denom;
    }

    fn get_power(n: u32) -> Rational {
	// n + 1/2 ( 1/2 comes from the sqrt(x) multiplicator )
        Rational::from((2*n + 1, 2))
    }

    fn integrate(coefficient: Rational, power: Rational) -> (Rational, Rational) {
        let powplus1 = power + 1;
        (coefficient / &powplus1, powplus1)
    }


    fn print(self) {
        println!("{:?}", self);
    }
}


#[cfg(test)]
pub fn calc_expected(numer: i32, denom: u32, exp: (u32, u32)) -> Rational {
    Rational::from((numer, denom * Integer::from(exp.0).pow(exp.1)))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coefficient() {
        let p = Params::new((1, 2), (1, 1), (1, 4));
        let ce = Term::get_coefficient(0, &p);
        assert_eq!(ce, Rational::from((1, 1)));
        let ce = Term::get_coefficient(1, &p);
        assert_eq!(ce, Rational::from((1, 2)));
        let ce = Term::get_coefficient(2, &p);
        assert_eq!(ce, Rational::from((-1, 8)));
        let ce = Term::get_coefficient(3, &p);
        assert_eq!(ce, Rational::from((1, 16)));
        let ce = Term::get_coefficient(4, &p);
        assert_eq!(ce, Rational::from((-5, 128)));
        let ce = Term::get_coefficient(5, &p);
        assert_eq!(ce, Rational::from((7, 256)));
    }

    #[test]
    fn test_power() {
        let pow = Term::get_power(5);
        assert_eq!(pow, Rational::from((11, 2)));
    }

    #[test]
    fn test_integrate() {
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let ce = Term::get_coefficient(5, &p);
        let pow = Term::get_power(5);
        let (ce, pow) = Term::integrate(ce, pow);
        let x = Myrat::pow(&p.limit, pow, 128);
        let calc = ce * x;
        let expected = calc_expected(-7, 1664, (2, 13));
        assert_eq!(calc.to_f64(), expected.to_f64());
    }

    #[test]
    fn test_term() {
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let t = <Term as MyTerm>::new(0, &p, 128);
        let expected = 	calc_expected(2, 3, (2, 3));
        assert_eq!(t, expected);
        let t = <Term as MyTerm>::new(1, &p, 128);
	let expected = calc_expected(-1, 5, (2, 5));    
        assert_eq!(t, expected);
        let t = <Term as MyTerm>::new(2, &p, 128);
	let expected = calc_expected(-1, 28, (2, 7));   
        assert_eq!(t, expected);
        let t = <Term as MyTerm>::new(3, &p, 128);
	let expected = calc_expected(-1, 72, (2, 9));   
        assert_eq!(t, expected);
        let t = <Term as MyTerm>::new(4, &p, 128);
	let expected = calc_expected(-5, 704, (2, 11)); 
        assert_eq!(t, expected);
        let t = <Term as MyTerm>::new(5, &p, 128);
	let expected = calc_expected(-7, 1664, (2, 13));
        assert_eq!(t, expected);
    }

}

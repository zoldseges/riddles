use rug::ops::Pow;
use rug::{Complete, Float, Integer, Rational};

pub type Params = SParams;
pub type Term = Rational;
pub type Pi = SPi;

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

pub trait Myterm {
    fn new(n: u32, p: &Params, prec: u32) -> Term;
    fn print(self);
}

impl Myterm for Term {
    fn new(n: u32, p: &Params, prec: u32) -> Term {
        // coefficient
        let ce = Term::get_coefficient(n, &p);
        let pow = Term::get_power(5, &p);
	println!("ce: {:?}", ce);
	println!("pow: {:?}", pow);
        let (ce, pow) = Term::integrate(ce, pow);
	println!("ce: {:?}", ce);
	println!("pow: {:?}", pow);
        ce * Myrat::pow(&p.limit, pow, prec).to_rational().unwrap()
    }

    fn print(self) {
        println!("{:?}", self);
    }
}

trait PrivTerm {
    fn get_coefficient(n: u32, p: &Params) -> Rational;
    fn get_power(n: u32, p: &Params) -> Rational;
    fn integrate(coefficient: Rational, power: Rational) -> (Rational, Rational);
}

impl PrivTerm for Term {
    fn get_coefficient(n: u32, p: &Params) -> Rational {
        let mut num: Rational = p.x.to_owned();
        let mut denom: Integer = Integer::new();

        for i in 0..n {
            num *= (&p.alpha - i).complete();
        }
	
	// TODO clean up if not needed
	// let sign =  &p.x.signum_ref().complete();
	// println!("{:?}", sign);
	// if sign != &0 {
	//     num *= sign;
	// }
	
        Integer::factorial(n).complete_into(&mut denom);

        return num / denom;
    }

    fn get_power(n: u32, p: &Params) -> Rational {
	// n + 1/2 ( 1/2 comes from the sqrt(x) multiplicator )
        Rational::from((2*n + 1, 2))
    }

    fn integrate(coefficient: Rational, power: Rational) -> (Rational, Rational) {
        let powplus1 = power + 1;
        (coefficient / &powplus1, powplus1)
    }
}

struct SPi {
    val: Float,
}

impl Pi {
    fn new(n: u32, prec: u32) -> Self {
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let val = Float::new(n) + Pi::get_term_sum(n, &p, prec);
        Pi { val }
    }

    fn get_term_sum(n: u32, p: &Params, prec: u32) -> Rational {
        let mut sum = Rational::new();
        for n in 0..=n {
            sum += <Term as Myterm>::new(n, &p, prec);
        }
        sum
    }
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

    // #[test]
    // fn test_power() {
    //     let p = Params::new((1, 2), (-1, 2), (1, 4));
    //     let pow = Term::get_power(5, &p);
    //     assert_eq!(pow, Rational::from((11, 2)));
    // }

    fn calc_expected(numer: i32, denom: u32, exp: (u32, u32)) -> Rational {
        Rational::from((numer, denom * Integer::from(exp.0).pow(exp.1)))
    }

    #[test]
    fn test_integrate() {
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let mut term: Term = Default::default();
        let ce = Term::get_coefficient(5, &p);
        let pow = Term::get_power(5, &p);
	println!("ce: {:?}", ce);
	println!("pow: {:?}", pow);
        let (ce, pow) = Term::integrate(ce, pow);
	println!("ce: {:?}", ce);
	println!("pow: {:?}", pow);
        let x = Myrat::pow(&p.limit, pow, 128);
        let calc = ce * x;
        let expected = calc_expected(-7, 1664, (2, 13));
        assert_eq!(calc.to_f64(), expected.to_f64());
    }

    #[test]
    fn test_term() {
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let t = <Term as Myterm>::new(0, &p, 128);
        let expected = 	calc_expected(2, 3, (2, 3));
        assert_eq!(t, expected);
        let t = <Term as Myterm>::new(1, &p, 128);
	let expected = calc_expected(-1, 5, (2, 5));    
        assert_eq!(t, expected);
        let t = <Term as Myterm>::new(2, &p, 128);
	let expected = calc_expected(-1, 28, (2, 7));   
        assert_eq!(t, expected);
        let t = <Term as Myterm>::new(3, &p, 128);
	let expected = calc_expected(-1, 72, (2, 9));   
        assert_eq!(t, expected);
        let t = <Term as Myterm>::new(4, &p, 128);
	let expected = calc_expected(-5, 704, (2, 11)); 
        assert_eq!(t, expected);
        let t = <Term as Myterm>::new(5, &p, 128);
	let expected = calc_expected(-7, 1664, (2, 13));
        assert_eq!(t, expected);
    }

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

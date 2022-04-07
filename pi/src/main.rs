// https://proofwiki.org/wiki/Newton%27s_Formula_for_Pi

use rug::{Integer, Rational, Float, Assign, Complete};

type Params = SParams;
type Term = STerm;

struct SParams {
    alpha: Rational,
    x: Rational,
}

impl SParams {
    fn new(alpha: (u32, u32), x: (u32, u32)) -> Params {
	let alpha = Rational::from(alpha);
	let x = Rational::from(x);
	let p = Params {alpha, x};
	p
    }
}

#[derive(Default)]
struct STerm {
    num: Rational,
    denom: Integer,
    power: Rational,
}

impl Term {
    
    fn new(n: u32, p: &Params) -> Term{
	let mut  t: Term = Default::default();
	t.num = Rational::from((1, 1));
	for i in 0..n {
	    t.num *= (&p.alpha- i).complete();
	}

	t.denom = Integer::new();
	Integer::factorial(n).complete_into(&mut t.denom);

	t.power = Rational::from((n, 1));
	t
    }
}

fn main() {
    let p = Params::new((1, 2),(1, 2));
    let mut t = Term::new(5, &p);
}

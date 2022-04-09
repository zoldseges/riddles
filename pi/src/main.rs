// https://proofwiki.org/wiki/Newton%27s_Formula_for_Pi

use pi::{Params, Term};

fn main() {
    let p = Params::new((1, 2), (1, 2), (1, 4));
    let mut t = Term::new(5, &p);
    println!("{:?}", t);
}

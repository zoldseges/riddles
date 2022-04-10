// https://proofwiki.org/wiki/Newton%27s_Formula_for_Pi
use pi::pi::{Pi, MyPi};

fn main() {
    println!("{:?}", <Pi as MyPi>::new(1000, 128));
}

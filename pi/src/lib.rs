pub mod pi;
mod utils;

pub mod bench {

    use rug::Float;
    use std::time::{Duration, Instant};

    use crate::pi::{MyPi, Pi};
    use crate::utils::{MyTerm, Params, Term};

    pub fn bench_new_term(n: u32, prec: u32, nsample: u32) -> (Duration, Term) {
        let mut samples: Vec<Duration> = Vec::with_capacity(nsample as usize);
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let mut term = Term::new();
        for _ in 0..nsample {
            let start = Instant::now();
            term = <Term as MyTerm>::new(n, &p, prec);
            let elapsed = start.elapsed();
            samples.push(elapsed);
        }
        (samples.iter().sum::<Duration>() / nsample, term)
    }

    pub fn bench_term_sum(limit: u32, prec: u32, nsample: u32) -> (Duration, Term) {
        let mut samples: Vec<Duration> = Vec::with_capacity(nsample as usize);
        let p = Params::new((1, 2), (-1, 1), (1, 4));
        let mut term = Term::new();
        for _ in 0..nsample {
            term = Term::new();
            let start = Instant::now();
            for n in 0..limit {
                term += <Term as MyTerm>::new(n, &p, prec);
            }
            let elapsed = start.elapsed();
            samples.push(elapsed);
        }
        (samples.iter().sum::<Duration>() / nsample, term)
    }

    pub fn bench_pi(n: u32, prec: u32, threaded: bool, nsample: u32) -> (Duration, Float) {
        let mut samples: Vec<Duration> = Vec::with_capacity(nsample as usize);
        let mut pi = Pi::new(128);
        for _ in 0..nsample {
            let start = Instant::now();
            pi = <Pi as MyPi>::new(n, prec, threaded);
            let elapsed = start.elapsed();
            samples.push(elapsed);
        }
        (samples.iter().sum::<Duration>() / nsample, pi)
    }
}

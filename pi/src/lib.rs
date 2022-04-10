mod utils;
pub mod pi;

pub mod bench {

    use crate::utils::{Term, MyTerm, Params};
    
    use std::time::{Duration, Instant};

    fn bench_new_term(n: u32, nsample: u32) -> Duration {
	let mut samples: Vec<Duration> = Vec::with_capacity(nsample as usize);
        let p = Params::new((1, 2), (-1, 1), (1, 4));
	for _ in 0..nsample {
	    let start = Instant::now();
	    <Term as MyTerm>::new(n, &p, 128);
	    let elapsed = start.elapsed();
	    samples.push(elapsed);
	}
	samples.iter().sum::<Duration>() / nsample
    }

    pub fn print_bench_new_term() {
	println!("{}", bench_new_term(1000, 100).as_micros());
    }
}
//     // pub fn bench_term_sum(n: u32, nsample: u32, prec: u32) -> Rational {
//     // 	let mut samples: Vec<Duration> = Vec::with_capacity(nsample as usize);
//     //     let p = Params::new((1, 2), (-1, 1), (1, 4));
//     // 	for i in 0..nsample {

//     // 	}
//     // }
// }

// #[bench]
// pub fn bench_new_term(n: u32, nsample: u32) -> Duration {
// 	let mut samples: Vec<Duration> = Vec::with_capacity(nsample as usize);
//     let p = Params::new((1, 2), (-1, 1), (1, 4));
// 	for i in 0..nsample {
// 	    let start = Instant::now();
// 	    <Term as MyTerm>::new(n, &p, 128);
// 	    let elapsed = start.elapsed();
// 	    samples.push(elapsed);
// 	}
// 	samples.iter().sum::<Duration>() / nsample
// }



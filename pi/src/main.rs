// https://proofwiki.org/wiki/Newton%27s_Formula_for_Pi
use getopts::Options;
use pi::pi::{MyPi, Pi};
use std::env;

// const UTF_PI: char = '\u{03c0}'; // greek pi;
const UTF_PI: char = '\u{1d77f}'; // mathematical utf pi;
const DEF_DEG: u32 = 256;
const DEF_PRECISION: u32 = 256;
const DEF_NSAMPLE: u32 = 1;
const DEF_SINGLETHREADED: bool = false;

fn print_usage(program: &str, opts: Options) {
    println!("This program calculates the value of {UTF_PI} using Newton's method");
    println!();
    let brief = format!("{} <NUM>", opts.short_usage(&program));
    println!("{}", opts.usage(&brief));
    println!("Input:");
    println!("    NUM                 {}", "degree of the polynomial calculating by");
}

// [-s] <n-terms> [-bsamplesize] [-bcomp "filename"] [-b] [-h]
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help");
    opts.optflag("s", "single", "use a single thread");
    opts.optflagopt("b", "bench", &format!("benchmark NUM calculations and prints its average (default NUM: {})", DEF_NSAMPLE), "NUM");
    opts.optopt("c", "cmp", "compares calculated {UTF_PI} value to the value specified in a file", "FILE");
    print_usage(&program, opts);
}

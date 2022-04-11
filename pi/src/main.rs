// https://proofwiki.org/wiki/Newton%27s_Formula_for_Pi
use exitcode;
use getopts::{Matches, Options};
use std::env;
use std::process::exit;

use pi::pi::{MyPi, Pi};

const METHOD: &str = "Newtons's method";
const PI_UTF: char = '\u{1d77f}'; // mathematical utf for pi;
const DEF_DEG: u32 = 256;
const DEF_PRECISION: u32 = 256;
const DEF_NSAMPLE: u32 = 1;
const DEF_SINGLETHREADED: bool = false;
const DEF_LOG_PATH: &str = "pi.log";

// This program calculates the value of 𝝿 using Newton's method
// Usage: target/debug/pi [-h] [-s] [-p] [-t [NUM]] [-b [FILE]] [-l [FILE]] DEG PREC

// Options:
//     -h, --help          Print this message
//     -s, --single        Use a single thread
//     -p, --progress      Show progress
//     -t, --time [NUM]    Get report on the average time being spent on
//                         calculating NUM times
//                         [default NUM]: 1
//     -b, --bprec [FILE]  Get report on the precision of the calculated 𝝿,
//                         provide a FILE containing the known value of 𝝿 to
//                         compare against
//                         [default FILE]: the first million decimals
//     -l, --log [FILE]    append benchmark log to FILE
//                         [default FILE]: "pi.log"

// Input:
//     DEG                 degree of the polynomial calculating by
//     PREC               floating point precision

fn run(m: Matches) {
    let is_single_threaded = m.opt_present("s");
    let bench_nsample = m.opt_default("b", &DEF_NSAMPLE.to_string());
}

fn short_usage(program: &str, opts: &Options) -> String{
    format!("{} DEG PREC", opts.short_usage(&program))
}
fn print_description(program: &str, opts: &Options) {
    let brief = short_usage(program, &opts);
    eprintln!("This program calculates the value of {PI_UTF} using {METHOD}");
    eprintln!("{}", opts.usage(&brief));
    eprintln!("Input:");
    eprintln!(
        "    DEG                 {}",
        "degree of the polynomial calculating by"
    );
    eprintln!(
        "    PREC               {}",
        "floating point precision"
    );
}

// TODO DEG and PREC could be related
// TODO implement infinite run
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "Print this message");
    opts.optflag("s", "single", "Use a single thread");
    opts.optflag("p", "progress", "Show progress");
    opts.optflagopt(
        "t",
        "time",
        &format!(
            "Get report on the average time being spent on calculating NUM times \n\
	     [default NUM]: {}",
            DEF_NSAMPLE
        ),
        "NUM",
    );
    opts.optflagopt(
        "b",
        "bprec",
        &format!(
            "Get report on the precision of the calculated {PI_UTF}, \
	     provide a FILE containing the known value of {PI_UTF} to compare against\n\
	     [default FILE]: the first million decimals"
        ),
        "FILE",
    );
    opts.optflagopt(
        "l",
        "log",
        &format!(
            "append benchmark log to FILE \n\
	     [default FILE]: \"{}\"",
            DEF_LOG_PATH
        ),
        "FILE",
    );
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
	    eprintln!("ERROR: {}", f.to_string());
	    eprintln!("{}", short_usage(&program, &opts));
	    exit(exitcode::USAGE);
	}
    };

    if matches.opt_present("h") {
        print_description(&program, &opts);
        return;
    }

    let deg = if matches.free.is_empty() {
	eprintln!("ERROR: argument DEG and PREC are required");
	eprintln!("{}", short_usage(&program, &opts));
        exit(exitcode::USAGE);
    } else {
        run(matches);
        exit(exitcode::OK);
    };
}

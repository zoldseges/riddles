// https://proofwiki.org/wiki/Newton%27s_Formula_for_Pi
use getopts::{Options, Matches};
use std::env;
use std::process::exit;
use exitcode;

use pi::pi::{MyPi, Pi};

const UTF_PI: char = '\u{1d77f}'; // mathematical utf pi;
const DEF_DEG: u32 = 256;
const DEF_PRECISION: u32 = 256;
const DEF_NSAMPLE: u32 = 1;
const DEF_SINGLETHREADED: bool = false;
const DEF_LOG_PATH: &str = "pi.log";

// Options:
//     -h, --help          print this help
//     -s, --single        use a single thread
//     -b, --bench [NUM]   benchmark NUM calculations and prints its average
//                         default NUM: 1
//     -c, --cmp FILE      compares calculated {UTF_PI} value to the value
//                         specified in a file
//     -p, --progress      show progress
//     -l, --log FILE      logs benchmark to FILE
//                         default: "pi.log.X" where X starts from 0
// Input:
//     NUM                 degree of the polynomial calculating by

fn run(m: Matches) {
    let single_threaded = m.opt_present("s");
    
}

fn print_usage(program: &str, opts: Options) {
    eprintln!("This program calculates the value of {UTF_PI} using Newton's method");
    let brief = format!("{} <NUM>", opts.short_usage(&program));
    eprintln!("{}", opts.usage(&brief));
    eprintln!("Input:");
    eprintln!("    NUM                 {}", "degree of the polynomial calculating by");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help");
    opts.optflag("s", "single", "use a single thread");
    opts.optflagopt("b", "bench", &format!("benchmark NUM calculations and prints its average \ndefault NUM: {}", DEF_NSAMPLE), "NUM");
    opts.optopt("c", "cmp", "compares calculated {UTF_PI} value to the value specified in a file", "FILE");
    opts.optflag("p", "progress", "show progress");
    opts.optopt("l", "log", &format!("logs benchmark to FILE \ndefault: \"{}.X\" where X starts from 0", DEF_LOG_PATH), "FILE");
    let matches = match opts.parse(&args[1..]) {
	Ok(m) => m,
	Err(f) => panic!("{}", f.to_string()),
    };

    if matches.opt_present("h") {
	print_usage(&program, opts);
	return;
    }

    let deg = if matches.free.is_empty() {
	print_usage(&program, opts);
	exit(exitcode::USAGE);
    } else {
	run(matches);
	exit(exitcode::OK);
    };
}

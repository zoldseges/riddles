// https://proofwiki.org/wiki/Newton%27s_Formula_for_Pi
use exitcode;
use std::env;
use std::fmt;
use std::fs::{read_to_string, File};
use std::path::Path;
use std::process::exit;

use getopts::{Matches, Options};
use shellexpand;

use pi::pi::{MyPi, Pi};

const METHOD: &str = "Newtons's method";
const PI_UTF: char = '\u{1d77f}'; // mathematical utf for pi;

const NAME_BENCH_TIME: &str = "bench-time";
const NAME_BENCH_PRECISION: &str = "bench-prec";
const NAME_BENCH_LOG: &str = "bench-log";
const DEF_NSAMPLE: &str = "1";
const DEF_LOG_PATH: &str = "pi.log";
const DEF_SINGLETHREADED: bool = false;
const PI_MIL: &str = include_str!("pi_mil.txt");

struct Args(pub Vec<String>);

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sf = self.0.iter().fold(String::new(), |mut acc, s| {
            if acc == "" {
                acc += s;
            } else {
                acc += &format!(" {}", s);
            }
            acc
        });
        write!(f, "{}", sf)
    }
}

fn short_usage(program: &str, opts: &Options) -> String {
    format!("{} <DEGREE> <PRECISION>", opts.short_usage(&program))
}

fn print_description(program: &str, opts: &Options) {
    let brief = short_usage(program, &opts);
    eprintln!("This program calculates the value of {PI_UTF} using {METHOD}");
    eprintln!("{}", opts.usage(&brief));
    eprintln!("Args:");
    eprintln!(
        "    <DEGREE>               {}",
        "degree of the polynomial calculating by"
    );
    eprintln!(
        "    <PRECISION>              {}",
        "floating point precision"
    );
}

fn log(deg: u32, fprec: u32, piprec: u32, timems: u32) {
    unimplemented!();
}

// expands matches into a tuple with error handling
fn expand_matches(
    matches: Matches,
) -> (
    u32,
    u32,
    bool,
    bool,
    Option<u32>,
    Option<String>,
    Option<File>,
) {
    let deg = match matches.free[0].parse::<u32>() {
        Ok(deg) => deg,
        Err(e) => {
            eprintln!("ERROR: {} <DEGREE>: \"{}\"", e, matches.free[0]);
            eprintln!("<DEGREE> and <PRECISION> must be integers");
            exit(exitcode::DATAERR);
        }
    };
    let prec = match matches.free[1].parse::<u32>() {
        Ok(prec) => prec,
        Err(e) => {
            eprintln!("ERROR: {} <PRECISION>: \"{}\"", e, matches.free[1]);
            eprintln!("<DEGREE> and <PRECISION> must be integers");
            exit(exitcode::DATAERR);
        }
    };
    let should_multithread: bool = !matches.opt_present("s");
    let should_show_progress: bool = matches.opt_present("p");
    let bench_time_nsample: Option<u32> = match matches.opt_default(NAME_BENCH_TIME, DEF_NSAMPLE) {
        Some(s) => match s.parse::<u32>() {
            Ok(n) => Some(n),
            Err(e) => {
                eprintln!("ERROR: {} argument \"{}\" - {}", NAME_BENCH_TIME, s, e);
                eprintln!("{}'s argument must be an integer or none", NAME_BENCH_TIME);
                exit(exitcode::DATAERR);
            }
        },
        None => None,
    };

    let bench_prec_cmp: Option<String> = if matches.opt_present(NAME_BENCH_PRECISION) {
        match matches.opt_str(NAME_BENCH_PRECISION) {
            Some(path) => {
                let expanded_path = shellexpand::tilde(&path).to_string();
                match read_to_string(Path::new(&expanded_path)) {
                    Ok(s) => Some(s),
                    Err(e) => {
                        eprintln!("ERROR: Couldn't open file \"{}\"", path);
                        eprintln!("{:?}", e.to_string());
                        exit(exitcode::DATAERR);
                    }
                }
            }
            None => Some(PI_MIL.to_string()),
        }
    } else {
        None
    };

    let bench_log_file: Option<File> = match matches.opt_default(NAME_BENCH_LOG, DEF_LOG_PATH) {
        Some(path) => {
            let expanded_path = shellexpand::tilde(&path).to_string();
            match File::create(Path::new(&expanded_path)) {
                Ok(f) => Some(f),
                Err(e) => {
                    eprintln!("ERROR: Couldn't open file \"{}\"", path);
                    eprintln!("{:?}", e.to_string());
                    exit(exitcode::DATAERR);
                }
            }
        }
        None => None,
    };

    (
        deg,
        prec,
        should_multithread,
        should_show_progress,
        bench_time_nsample,
        bench_prec_cmp,
        bench_log_file,
    )
}

fn run(matches: Matches) {
    // parse deg and prec
    let (
        deg,
        prec,
        should_multithread,
        should_show_progress,
        bench_time_nsample,
        bench_prec_cmp,
        bench_log_file,
    ) = expand_matches(matches);

    println!("deg: {:?}", deg);
    println!("prec: {:?}", prec);
    println!("should_multithread: {:?}", should_multithread);
    println!("should_show_progress: {:?}", should_show_progress);
    println!("bench_time_nsample: {:?}", bench_time_nsample);
    if let Some(s) = bench_prec_cmp {
	println!("bench_prec_cmp: {:?}", &s[0..6]);
    } else {
	println!("bench_prec_cmp: {:?}", None::<String>);
    }
    println!("bench_log_file: {:?}", bench_log_file);
}

// TODO DEG and PREC could be related
// TODO implement infinite run
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "Print this message.");
    opts.optflag("s", "single", "Use a single thread.");
    opts.optflag("p", "progress", "Show progress.");
    opts.optflagopt(
        "",
        NAME_BENCH_TIME,
        &format!(
            "Get report on the average time being spent on approximating {PI_UTF}. \
	     Computation is going to run NUM times.\n\
	     [default NUM]: {}",
            DEF_NSAMPLE
        ),
        "=NUM",
    );
    opts.optflagopt(
        "",
        NAME_BENCH_PRECISION,
        &format!(
            "Get report on the precision of the calculated {PI_UTF}.\n\
	     If FILE provided, the calculated value is going to be compared against it.\n\
	     [default FILE]: the first million decimals of {PI_UTF}"
        ),
        "=FILE",
    );
    opts.optflagopt(
        "",
        NAME_BENCH_LOG,
        &format!(
            "Append reports to FILE.\n\
	     [default FILE]: {}",
            DEF_LOG_PATH
        ),
        "=FILE",
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

    match matches.free.len() {
        2 => {
            // check if theres benchmark set to be logged
            if matches.opt_present(NAME_BENCH_LOG) {
                if !(matches.opt_present(NAME_BENCH_TIME)
                    || matches.opt_present(NAME_BENCH_PRECISION))
                {
                    eprintln!("ERROR: There is nothing to log, because no benchmark is set.");
                    eprintln!(
                        "Consider setting \"--{}\" or \"--{}\" or both.",
                        NAME_BENCH_TIME, NAME_BENCH_PRECISION
                    );
                    eprintln!("{}", short_usage(&program, &opts));
                    exit(exitcode::USAGE);
                }
            }
            run(matches);
            exit(exitcode::OK);
        }
        _ => {
            eprintln!(
                "ERROR: exactly 2 main arguments must be given \"<DEC> <PRECISION>\", \
		 you gave \"{}\"",
                Args(matches.free)
            );
            eprintln!("{}", short_usage(&program, &opts));
            exit(exitcode::USAGE);
        }
    };
}

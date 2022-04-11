use pi::bench::*;
use rug::Float;
use std::env;

const PRECISION: u32 = 128;
const NUMBER_OF_SAMPLES: u32 = 10;

fn validator(args: Vec<String>) -> Result<(u32, u32, u32, bool), String> {
    // default values
    let mut p = PRECISION;
    let mut ns = NUMBER_OF_SAMPLES;
    let mut sum = false;
    let len = args.len();

    if len < 2 {
        return Err("No arguments".to_string());
    }

    let n = match args[1].parse::<i32>() {
        Ok(n) => {
            if n >= 0 {
                n as u32
            } else {
                sum = true;
                n.abs() as u32
            }
        }
        Err(_) => return Err(format!("Couldn't parse {}", args[1])),
    };

    if len > 2 {
        p = match args[2].parse::<u32>() {
            Ok(n) => n,
            Err(_) => return Err(format!("Couldn't parse {}", args[2])),
        };
    }

    if len > 3 {
        ns = match args[3].parse::<u32>() {
            Ok(n) => n,
            Err(_) => return Err(format!("Couldn't parse {}", args[3])),
        };
    }

    Ok((n, p, ns, sum))
}

fn run(n: u32, prec: u32, ns: u32, sum: bool) {
    if sum {
        let (duration, sum) = bench_term_sum(n, prec, ns);
        println!(
            "n: {}\t{:.6}s\tsum: {}",
            n,
            duration.as_micros() as f32 / 1_000_000 as f32,
            Float::with_val(prec, sum)
        );
    } else {
        let (duration, val) = bench_new_term(n, prec, ns);
        println!(
            "n: {}\t{:.6}s\tval: {}",
            n,
            duration.as_micros() as f32 / 1_000_000 as f32,
            Float::with_val(prec, val)
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = args[0].clone();
    match validator(args) {
        Err(e) => {
            println!("Error: {}", e);
            println!(
                "Usage: {}  <n-th term> | -<to nth term> [precision] [number of samples]",
                prog
            );
            println!("[precision]         default: {}", PRECISION);
            println!("[number of samples] default: {}", NUMBER_OF_SAMPLES);
        }
        Ok((n, p, ns, sum)) => run(n, p, ns, sum),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator() {
        let args = Vec::new();
        let result = validator(args);
        let expected = Err("No arguments".to_string());
        assert_eq!(expected, result);

        let args = vec!["".to_string()];
        let result = validator(args);
        let expected = Err("No arguments".to_string());
        assert_eq!(expected, result);

        let args = vec!["".to_string(), "0.01".to_string()];
        let result = validator(args);
        let expected = Err("Couldn't parse 0.01".to_string());
        assert_eq!(expected, result);

        let args = vec!["".to_string(), "10".to_string()];
        let result = validator(args);
        let expected = Ok((10, 128, 10, false));
        assert_eq!(expected, result);

        let args = vec!["".to_string(), "10".to_string(), "-23".to_string()];
        let result = validator(args);
        let expected = Err("Couldn't parse -23".to_string());
        assert_eq!(expected, result);

        let args = vec!["".to_string(), "10".to_string(), "256".to_string()];
        let result = validator(args);
        let expected = Ok((10, 256, 10, false));
        assert_eq!(expected, result);

        let args = vec![
            "".to_string(),
            "10".to_string(),
            "256".to_string(),
            "2342346456464645".to_string(),
        ];
        let result = validator(args);
        let expected = Err("Couldn't parse 2342346456464645".to_string());
        assert_eq!(expected, result);

        let args = vec![
            "".to_string(),
            "32".to_string(),
            "256".to_string(),
            "1000".to_string(),
        ];
        let expected = Ok((32, 256, 1000, false));
        let result = validator(args);
        assert_eq!(expected, result);
    }
}

use std::env;

fn print_usage(prog: &str) {
}

fn validator(args: Vec<String>) -> Result<(u32, u32, u32), String>
{
    // default values
    let mut p = 128;
    let mut ns = 10;
    let len = args.len();
    
    if len < 2 {
	return Err("No arguments".to_string());
    }

    let n = match args[1].parse::<u32>() {
	Ok(n) => n,
	Err(_) => {
	    -1;
	    return Err(format!("Couldn't parse {}", args[1]));
	},
    };

    if len > 2 {
	p = match args[2].parse::<u32>() {
	    Ok(n) => n,
	    Err(_) => {
		-1;
		return Err(format!("Couldn't parse {}", args[2]));
	    },
	};
    }

    if len > 3 {
	ns = match args[3].parse::<u32>() {
	    Ok(n) => n,
	    Err(_) => {
		-1;
		return Err(format!("Couldn't parse {}", args[3]));
	    },
	};
    }

    
    Ok((n, p, ns))
}

fn run(n: u32, prec: u32, ns: u32) {
    print!("yaay! it runs!");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = args[0].clone();
    match validator(args) {
	Err(e) => {
	    println!("Error: {}", e);
	    println!("Usage: {} <n-th term> [precision] [number of samples]", prog);
	}
	Ok((n, p, ns)) => run(n, p, ns),
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
	let expected = Ok((10, 128, 10));
	assert_eq!(expected, result);

	let args = vec!["".to_string(), "10".to_string(), "-23".to_string()];
	let result = validator(args);
	let expected = Err("Couldn't parse -23".to_string());
	assert_eq!(expected, result);

	let args = vec!["".to_string(), "10".to_string(), "256".to_string()];
	let result = validator(args);
	let expected = Ok((10, 256, 10));
	assert_eq!(expected, result);

	let args = vec!["".to_string(), "10".to_string(), "256".to_string(), "2342346456464645".to_string()];
	let result = validator(args);
	let expected = Err("Couldn't parse 2342346456464645".to_string());
	assert_eq!(expected, result);

    	let args = vec!["".to_string(), "32".to_string(), "256".to_string(), "1000".to_string()];
	let expected = Ok((32, 256, 1000));
	let result = validator(args);
	assert_eq!(expected, result);
    }
}

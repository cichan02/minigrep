use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from_slice(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    file_path: String,
    query: String,
    ignore_case: bool
}

impl Config {
    fn new(file_path: &str, query: &str, ignore_case: bool) -> Self {
        Self {
            file_path: file_path.to_string(),
            query: query.to_string(),
            ignore_case,
        }
    }

    fn from_slice(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self::new( &args[1],  &args[2], ignore_case))
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content: String = fs::read_to_string(config.file_path)?;

    let res = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in res {
        println!("{line}");
    }

    Ok(())
}
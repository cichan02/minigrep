use std::env;
use std::error::Error;
use std::fs;
use std::process;
use minigrep::{search, search_case_insensitive};

fn main() {
    let config = Config::from_slice(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}

struct Config {
    file_path: String,
    query: String,
    ignore_case: bool
}

impl Config {
    fn new(file_path: String, query: String, ignore_case: bool) -> Self {
        Self {
            file_path,
            query,
            ignore_case,
        }
    }

    fn from_slice(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // skip the name of project
        args.next();

        let file_path = match args.next() {
            Some(s) => s,
            None => return Err("Din't get a file path"),
        };

        let query = match args.next() {
            Some(s) => s,
            None => return Err("Din't get a query"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self::new(file_path, query, ignore_case))
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content: String = fs::read_to_string(config.file_path)?;

    if config.ignore_case { 
        print_lines(search_case_insensitive(&config.query, &content));
    } else {
        print_lines(search(&config.query, &content));
    }

    Ok(())
}

fn print_lines<'a>(lines: impl Iterator<Item = &'a str>) {
    for line in lines {
        println!("{line}");
    }
}
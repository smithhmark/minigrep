use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("could not parse args:{}", err);
        process::exit(1);
    });
    println!("Searching for:{}", config.query);
    println!("Searching in file:{}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error:{}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)
        .expect("Oh no, file problem!");

    println!("Contents:\n{}", contents);

    Ok(()) //indiomatic: means "this function called for side-effects"
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}

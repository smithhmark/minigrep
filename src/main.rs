use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_args(&args);
    println!("Searching for:{}", config.query);
    println!("Searching in file:{}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Oh no, file problem!");

    println!("Contents:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_args(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {query, filename}
}

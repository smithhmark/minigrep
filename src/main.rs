use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_args(&args);
    println!("Searching for:{}", query);
    println!("Searching in file:{}", filename);

    let contents = fs::read_to_string(filename).expect("Oh no, file problem!");

    println!("Contents:\n{}", contents);
}

fn parse_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)
        .expect("Oh no, file problem!");

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(()) //indiomatic: means "this function called for side-effects"
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insenitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename, case_sensitive: false})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape!";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape!";

        assert_eq!(vec!["safe, fast, productive.", "Duct tape!"],
                   search_case_insenitive(query, contents));
    }

    #[test]
    fn parse_cfg() {
        let args = [
            "prg".to_string(), 
            "query".to_string(),
            "filename".to_string()];
        let query = "query".to_string();
        let filename = "filename".to_string();
        let expected = Config { query, filename, case_sensitive: false};
        let got = Config::new(&args[..]).unwrap();
        assert_eq!(expected, got)
    }
}

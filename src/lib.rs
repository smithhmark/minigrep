use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)
        .expect("Oh no, file problem!");

    println!("Contents:\n{}", contents);

    Ok(()) //indiomatic: means "this function called for side-effects"
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cfg() {
        let args = [
            "prg".to_string(), 
            "query".to_string(),
            "filename".to_string()];
        let query = "query".to_string();
        let filename = "filename".to_string();
        let expected = Config { query, filename};
        let got = Config::new(&args[..]).unwrap();
        assert_eq!(expected, got)
    }
}

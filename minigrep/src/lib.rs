use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("text: {}", contents);
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("fuck!");
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
    fn one_result() {
        let query = "fuck";
        let contents = "\
Rust:motherfucker
safe, apple, pear";

        assert_eq!(
            vec!["safe, fase, productive."],
            search(query, contents)
        );
    }
}

pub fn search<'a>(query: &str) -> &str {
    "a"
}

use std::error::Error;
use std::fs::read_to_string;
use std::env::var;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not Enough Args!");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        let case_sensitive = var("CASE_INSENSITIVE").is_err();
        
        return Ok(Config { query, file_name , case_sensitive});
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents =
        read_to_string(config.file_name)?;

    let result = if config.case_sensitive {
        search(config.query.as_str(), file_contents.as_str())
    } else {
        search_cis(config.query.as_str(), file_contents.as_str())
    };

    println!("Searching for `{}`", config.query);

    if result.is_empty() {
        println!("Found no item matching `{}`", config.query);
    } else {
        for line in result {
            print!("{line}");
        }
    }

    return Ok(());
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn search_cis<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query.as_str()) {
            results.push(line);
        }
    }

    return results;
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
Pick Three,
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick Three,
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_cis(query, contents));
    }
}
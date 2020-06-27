use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let config = Config::new(&[
            String::from("minigrep"),
            String::from("test_arg"),
            String::from("test_filename"),
        ])
        .unwrap();

        assert_eq!(config.filename, "test_filename");
        assert_eq!(config.query, "test_arg");
    }

    #[test]
    fn new_error_when_not_enough_args() {
        let result = Config::new(&[String::from("minigrep"), String::from("test_arg")]);
        assert!(
            result.is_err(),
            "New did not return error with two arguments"
        );
    }

    #[test]
    fn run_fails_when_file_is_missing() {
        let config = Config::new(&[
            String::from("minigrep"),
            String::from("test_arg"),
            String::from("Non_existing_file"),
        ])
        .unwrap();
        let result = run(config);
        assert!(
            result.is_err(),
            "Run did not return error when file is missing"
        )
    }

    #[test]
    fn search_sensitive() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }

    #[test]
    fn search_insensitive() {
        let query = "rUsT";
        let contents = "
Rust:
safe, fast, productive
Pick three.
Trust me";
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, contents)
        );
    }
}

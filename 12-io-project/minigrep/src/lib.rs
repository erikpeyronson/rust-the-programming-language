use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };

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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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
        let args = vec![
            String::from("minigrep"),
            String::from("test_arg"),
            String::from("test_filename"),
        ];

        let config = Config::new(args.into_iter()).unwrap();

        assert_eq!(config.filename, "test_filename");
        assert_eq!(config.query, "test_arg");
    }

    #[test]
    fn new_error_when_not_enough_args() {
        let args = vec![String::from("minigrep"), String::from("test_arg")];
        let result = Config::new(args.into_iter());
        assert!(
            result.is_err(),
            "New did not return error with two arguments"
        );
    }

    #[test]
    fn run_fails_when_file_is_missing() {
        let args = vec![
            String::from("minigrep"),
            String::from("test_arg"),
            String::from("non_existing_file"),
        ];
        let config = Config::new(args.into_iter()).unwrap();
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

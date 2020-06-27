use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
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
    fn test_new_error_when_not_enough_args() {
        let result = Config::new(&[String::from("minigrep"), String::from("test_arg")]);
        assert!(
            result.is_err(),
            "New did not return error with two arguments"
        );
    }

    #[test]
    fn test_run_fails_when_file_is_missing() {
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
}

use std::env;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    fn new(query: String, file_path: String) -> Config {
        Config { query, file_path }
    }

    pub fn get_input_config() -> Config {
        let args: Vec<String> = env::args().collect();

        Config::build(&args).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            process::exit(1)
        })
    }

    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Usage: rtw-grep <pattern> <path>");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config::new(query, file_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = Config::new(String::from("Hello"), String::from("file.txt"));

        assert_eq!(config.query, "Hello");
        assert_eq!(config.file_path, "file.txt");
    }

    #[test]
    fn test_build() {
        let args = vec![
            String::from("program"),
            String::from("query"),
            String::from("file.txt"),
        ];

        let config = Config::build(&args).unwrap();
        
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file.txt");
    }

    #[test]
    fn test_build_not_enough_args() {
        let args = vec![String::from("program"), String::from("query")];

        assert!(Config::build(&args).is_err())
    }

    #[test]
    fn test_build_error_message() {
        let args = vec![String::from("program")];
        let result = Config::build(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Usage: rtw-grep <pattern> <path>".to_string());
    }
}

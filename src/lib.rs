use std::fs;
use std::error::Error;

pub struct Config {
    filename: String,
    word: String,
    ignore_case: bool
}

impl Config {
    pub fn word(&self) -> &String {
        &self.word
    }
    pub fn filename(&self) -> &String {
        &self.filename
    }
    pub fn new(filename: String, word: String, ignore_case: bool) -> Config {
        Config {
            filename,
            word,
            ignore_case
        }
    }
    
    pub fn build(params: &[String]) -> Result<Config, &'static str> {
        let mut filename: Option<String> = None;
        let mut word: Option<String> = None;
        let mut ignore_case = false;
        let mut i = 0;
        if params.len() < 4 {
            return Err("Not enough arguments");
        }
        while i < params.len() {
            match params[i].as_str() {
                "--file" | "-f" => {
                    if i + 1 >= params.len() {
                        return Err("Missing filename");
                    }
                    filename = Some(params[i+1].clone());
                    i += 1;
                },
                "--word" | "-w" => {
                    if i + 1 >= params.len() {
                        return Err("Missing word");
                    }
                    word = Some(params[i+1].clone());
                    i += 1;
                },
                "--ignore_case" | "-ic" => {
                    ignore_case = true;
                },
                _ => {}
            }
            i += 1;
        }
        match (filename, word, ignore_case) {
            (Some(f), Some(w), ic) => return Ok(Config::new(f, w, ic)),
            _ => return Err("Some arguments are missed")
        }
    }
}

fn search_sensitive<'a>(word: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(word) {
            result.push(line.trim());
        } 
    }
    result
}

fn search_insensitive<'a>(word: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let word = word.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&word) {
            result.push(line.trim());
        } 
    }
    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = if config.ignore_case == true {
        search_insensitive(&config.word, &contents)
    } else {
        search_sensitive(&config.word, &contents)
    };
    for line in result {
        println!("{line}");
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_config() {
        let params = Config::new(
            String::from("test.txt"),
            String::from("test"),
            false
        );
        assert_eq!(params.filename, "test.txt");
        assert_eq!(params.word, "test");
    }

    #[test]
    fn parse_full_args() {
        let args = [
            String::from("--ignore_case"),
            String::from("--file"),
            String::from("test.txt"),
            String::from("--word"),
            String::from("test")
        ];
        let params = Config::build(&args).unwrap();
        assert_eq!(params.filename, "test.txt");
        assert_eq!(params.word, "test");
        assert_eq!(params.ignore_case, true);
    }

    #[test]
    fn parse_short_args() {
        let args = [
            String::from("-f"),
            String::from("test.txt"),
            String::from("-w"),
            String::from("test"),
            String::from("-ic")
        ];
        let params = Config::build(&args).unwrap();
        assert_eq!(params.filename, "test.txt");
        assert_eq!(params.word, "test");
        assert_eq!(params.ignore_case, true);
    }

    #[test]
    fn parse_missing() {
        let args = [
            String::from("test.txt"),
            String::from("--word"),
            String::from("test"),
            String::from("--file"),
        ];
        let params = Config::build(&args);
        assert!(params.is_err());
    }

    #[test]
    fn parse_not_enough() {
        let args = [
            String::from("--file"),
            String::from("test.txt"),
        ];
        let params = Config::build(&args);
        assert!(params.is_err());
    }

    #[test]
    fn search_success() {
        let contents = "\
            Cat eat milk.
            Dogs eat meat.
            Cat playes with balls.
        ";
        assert_eq!(
            vec![
                "Cat eat milk.",
                "Cat playes with balls."
            ],
            search_insensitive("cat", contents)
        );
        assert_eq!(
            vec![
                "Cat eat milk.",
                "Cat playes with balls."
            ],
            search_sensitive("Cat", contents)
        );
    }
}

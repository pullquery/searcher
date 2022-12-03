use std::error::Error;
use std::fs::File;
use std::io::Read;

use super::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename).expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");

    if config.case_sensitive {
        for line in search_case_sensitive(&config.query, &content) {
            println!("{}", line)
        }
    } else {
        for line in search_case_insensitive(&config.query, &content) {
            println!("{}", line)
        }
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }

    result
}

use std::error::Error;
use std::fs::File;
use std::io::Read;

use super::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename).expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read file");

    if config.case_sensitive {
        search_case_sensitive(&config.query, &content)
            .iter()
            .for_each(|line| println!("{}", line))
    } else {
        search_case_insensitive(&config.query, &content)
            .iter()
            .for_each(|line| println!("{}", line))
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

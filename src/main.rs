use std::env;
use std::process;

mod searcher;
use searcher::config::Config;
use searcher::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = search::run(config) {
        eprintln!("running error: {}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use crate::searcher::search;

    #[test]
    fn test_search() {
        let query = "safe";
        let content = "
Rust:
safe, fast, productive.
Pick three.
";

        assert_eq!(
            vec!["safe, fast, productive."],
            search::search_case_sensitive(query, content)
        );
    }

    #[test]
    fn test_case_insensitive() {
        let query = "ruSt";
        let content = "
Rust:
safe, fast, productive.
Pick three.
";

        assert_eq!(
            vec!["Rust:"],
            search::search_case_insensitive(query, content)
        );
    }
}

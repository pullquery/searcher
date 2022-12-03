use std::env;
use std::process;

mod searcher;
use searcher::config::Config;
use searcher::search;

mod test;

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

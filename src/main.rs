extern crate searcher;

use std::env;
use std::process;

use searcher::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = searcher::run(config) {
        eprintln!("running error: {}", e);
        process::exit(1);
    }
}

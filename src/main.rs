use std::env;
use std::process;
use rust_book_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|_| {
        process::exit(1);
    });

    if let Err(_) = rust_book_minigrep::run(config) {
        process::exit(1);
    }
}

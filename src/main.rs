use std::{env, process};

use minigrep::Config;
use minigrep::run;

/// cargo run -- livelong poem.txt
/// IGNORE_CASE=1 cargo run -- Banish poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


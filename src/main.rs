use minigrep::Config;
use std::{env, process};

fn main() {
    let delim = "-------------";
    println!("{}", delim);
    // will panic on invalid UTF8
    // std::env::args_os will not, but depend on OS, type will be OsString
    let args: Vec<String> = env::args().collect();
    // 0th - executable name
    // 1st param
    // 2nd param
    // ... param
    // dbg!(args);
    let config = Config::make(&args).unwrap_or_else(|err| {
        eprintln!("Arguments parsing error: {}", err);
        println!("{}", delim);
        process::exit(1)
    });

    println!("Searching for `{}`", config.query);
    println!("In file `{}`", config.file_path);

    // simplified pattern matching, actually partial handling
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        println!("{}", delim);
        process::exit(1)
    }

    println!("{}", delim);
}

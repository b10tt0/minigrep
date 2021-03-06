use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    // using 'if let' here becaues run returns () in success, we only care about detecting an err
    if let Err(e) = minigrep::run(config) { 
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
} 

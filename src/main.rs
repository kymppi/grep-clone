use std::env;
use std::process;

use grepclone::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = grepclone::run(config) {
        eprintln!("App error: {}", e);
        process::exit(1);
    }
}

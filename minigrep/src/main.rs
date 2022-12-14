use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem with parsing arguments: {err}");
        process::exit(1);
        }
    );

    if let Err(error) = minigrep::run(&config){
        eprintln!("Parsing error occured: {error}");
        process::exit(1);
    }
}

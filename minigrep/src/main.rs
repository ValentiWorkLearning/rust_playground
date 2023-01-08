use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let cmd_args:Vec<String> = env::args().collect();

    let config = Config::new(&cmd_args).unwrap_or_else(|err|{
        eprintln!("Problem with parsing arguments: {err}");
        process::exit(1);
        }
    );

    dbg!(cmd_args.clone());

    if let Err(error) = minigrep::run(&config){
        eprintln!("Parsing error occured: {error}");
        process::exit(1);
    }
}

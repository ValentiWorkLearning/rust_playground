use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let cmd_args:Vec<String> = env::args().collect();

    let config = Config::new(&cmd_args);

    
    dbg!(cmd_args.clone());

    if let Err(error) = minigrep::run(&config.unwrap()){
        println!("Parsing error occured: {error}");
        process::exit(1);
    }
}

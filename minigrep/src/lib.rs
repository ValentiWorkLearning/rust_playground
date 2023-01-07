use std::error::Error;
use std::fs;

pub struct Config{
    pub query:String,
    pub file_path:String
}

pub fn _parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let file_path = &args[2];

    Config{query:query.to_string(), file_path:file_path.to_string()}
}

impl Config{
    pub fn new(args: &[String])->Result<Config,&'static str>
    {
        const ARGS_COUNT:usize = 3;
        if args.len()<ARGS_COUNT {
            return Err("Arguments count should be 2 - a command and a filepath")
        }

        let query = &args[1];
        let file_path = &args[2];

        Ok(Config{query:query.to_string(), file_path:file_path.to_string()})
    }
}


pub fn run(config: &Config)->Result<(),Box<dyn Error>>{
    
    let program_query = &config.query;
    let filepath = &config.file_path;

    println!("requested action: {program_query}");
    println!("filepath is: {filepath}");

    let file_content = fs::read_to_string(&filepath)?;
    
    println!("File content is: {file_content}");
    Ok(())
}
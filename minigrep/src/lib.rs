use std::error::Error;
use std::{fs, vec,env};

pub struct Config{
    pub query:String,
    pub file_path:String,
    pub search_mode:SearchingMode
}
#[derive(Clone,Copy)]
pub enum SearchingMode {
    CaseSensitive,
    CaseInsensitive
}

pub fn _parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let file_path = &args[2];

    Config{query:query.to_string(), file_path:file_path.to_string(), search_mode:SearchingMode::CaseSensitive}
}

impl Config{
    pub fn new(mut args: impl Iterator<Item = String>)->Result<Config,&'static str>
    {
        args.next();

        let query = match args.next() {
            Some(arg)=>arg,
            None=>return Err("Didn't get the query string")
        };

        let file_path = match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get the file path")
        };

        let search_mode_env =
        match env::var("IGNORE_CASE").is_ok(){
            true=> SearchingMode::CaseInsensitive,
            false=>SearchingMode::CaseSensitive
        };

        Ok(Config{query:query.to_string(), file_path:file_path.to_string(),search_mode: search_mode_env})
    }
}


pub fn run(config: &Config)->Result<(),Box<dyn Error>>{
    
    let program_query = &config.query;
    let filepath = &config.file_path;

    println!("requested action: {program_query}");
    println!("filepath is: {filepath}");

    let file_content = fs::read_to_string(&filepath)?;
    
    println!("File content is: {file_content}");

    println!("==grep result begin==");

    for line in search(&config.query, &file_content,config.search_mode){
        println!("{line}")
    }

    println!("==grep result end==");

    Ok(())
}



pub fn search<'a>(query: &str, contents: &'a str, search_strategy: SearchingMode)-> Vec<&'a str>{

    let lowercase_query = query.to_lowercase();
    //Modern implementation is:
    contents.lines().filter(
        |line|
        {
            match search_strategy {
                SearchingMode::CaseInsensitive=>{
                    line.to_lowercase().contains(&lowercase_query)
                }
                SearchingMode::CaseSensitive=>{
                    line.contains(&query)
                }
            }
        }
    ).collect()

    //    let mut results = Vec::new();
    // for line in contents.lines(){
    //     match search_strategy {
    //         SearchingMode::CaseInsensitive=>{
    //             let lowercase_query = query.to_lowercase();
    //             if line.to_lowercase().contains(&lowercase_query){
    //                 results.push(line);
    //             }
    //         }
    //         SearchingMode::CaseSensitive=>{
    //             if line.contains(query){
    //                 results.push(line);
    //             }
    //         }
    //     }
    // }

    // results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three";

        assert_eq!(vec!["safe, fast, productive."], search(query,contents,SearchingMode::CaseSensitive));
    }


    #[test]
    fn no_results(){
        let query = "none-existing-string";
        let contents = "\
Rust:
safe, fast, productive.
Pick three";
        let empty_vec:Vec<&str> = Vec::new();

        assert_eq!(empty_vec, search(query, contents,SearchingMode::CaseSensitive));
    }


    #[test]
    fn case_insensitive(){
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three";

        assert_eq!(vec!["Rust:"], search(query,contents,SearchingMode::CaseInsensitive));
    }
}
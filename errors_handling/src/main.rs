use std::error::Error;
use std::fs::File;

pub mod panic_demo;
fn main()-> Result<(), Box<dyn Error>>{
    panic_demo::panic_demo::run_panic_demo();
    let _file = File::open("text.txt")?;

    Ok(())
}

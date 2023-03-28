use crate::{demo_trait};
use hello_macro;

#[macro_export]
macro_rules! vec_demo {
    ($($x:expr),*) => {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    };
}


use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}


pub struct MacroDemo
{
}
impl demo_trait::Demo for MacroDemo{
    fn run_demo(&self) {
        
    }
}
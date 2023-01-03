
use crate::traits_demo::{Summary,Notice};

use std::fmt::{Display, format};

pub struct TgNews{
    pub author:String,
    pub message:String
}

impl Summary for TgNews{
    fn summarize(&self)->String {
        format!("News from tg: {} by {}",self.message,self.author)
    }
    fn whoami(&self)->String {
        format!("I'm {}",self.author)
    }
}

impl Notice for TgNews{
 fn make_notice(&self) {
     println!("Lol, mega notice: {}", self.message);
 }
}

// custom display formatter:
// impl <T: Display> ToString for T {
//     fn to_string(&self) -> String {
        
//     }     
// }

impl Display for TgNews{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"I'm strigified! {},{}",self.author,self.message)
    }
}

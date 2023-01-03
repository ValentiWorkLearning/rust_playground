pub trait Summary{
    fn summarize(&self)->String;

    fn default_method_implementation(&self){
        println!("So that you have to implement me");
    }

    fn whoami(&self)->String;
}

pub trait Notice {
    fn make_notice(&self);
}

use crate::tweet_summary::NewsFromTwitter;
use crate::news_from_tg::TgNews;

pub fn test_demo(){
    let twitter_news = NewsFromTwitter{
        headline:"hello world".to_string(),
        location:"USA".to_string(),
        author:"Lorem Ipsum".to_string(),
        content:"Something meaningful".to_string()
    };

    let tg_news = TgNews{
        message:"foo bar".to_string(),
        author:"Not me".to_string()
    };

}


pub fn return_summarizable()-> impl Summary{
    TgNews{
        message:"foo bar".to_string(),
        author:"Not me".to_string()
    }
}


// trait bounds sample

use std::fmt::Display;


struct Pair<T>{
    x:T,
    y:T
}

impl<T> Pair<T>{
     fn new(x:T,y:T)->Self{
        Self{x,y}
     }
}

impl<T:Display+PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x > self.y{
            println!("Greater!");
        }
        else {
            println!("Lower!");
        }
    }
}
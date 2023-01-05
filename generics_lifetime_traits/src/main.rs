
mod generics_demo;
mod tweet_summary;
mod news_from_tg;
mod traits_demo;
mod lifetimes_demo;

use traits_demo::{Summary,Notice};
use news_from_tg::{TgNews};
use tweet_summary::{NewsFromTwitter};

//general syntax
fn make_shout_alarm(news:&impl Summary){
    println!("Breaking news! {}", news.summarize());
}

//trait bound syntax
fn general_alarm<T:Summary>(news: &T){
    println!("Generic break! {}", news.summarize());
}

// cobination of traits
fn general_omega_alarm<T:Summary+Notice>(news: &T){
    news.make_notice();
}

fn general_omega_alrm_with_another_syntax(news: &(impl Summary+Notice)){
    news.make_notice();
}


// with clause
fn general_clause_function<T,U>(first:&T, second:&U)->String
where T:Summary , U:Notice + Summary
{
    first.summarize();
    second.make_notice();
    second.summarize()
}

use lifetimes_demo::lifetime;

fn main() {
    generics_demo::run_generics_demo();
    traits_demo::test_demo();

    let tg_news = TgNews{
        message:"foo bar".to_string(),
        author:"Not me".to_string()
    };

    let twitter_news = NewsFromTwitter{
        headline:"hello world".to_string(),
        location:"USA".to_string(),
        author:"Lorem Ipsum".to_string(),
        content:"Something meaningful".to_string()
    };

    tg_news.summarize();
    tg_news.default_method_implementation();
    tg_news.whoami();

    make_shout_alarm(&tg_news);
    make_shout_alarm(&twitter_news);

    general_alarm(&tg_news);
    general_alarm(&twitter_news);
    general_omega_alarm(&tg_news);
    general_omega_alrm_with_another_syntax(&tg_news);


    //can't call because of the missing trait implementation
    //general_omega_alarm(&twitter_news);

    // concept requires doesn't match
    //general_clause_function(&tg_news, &twitter_news);
    general_clause_function(&twitter_news, &tg_news);

    let something_cool = traits_demo::return_summarizable();
    something_cool.whoami();

    println!("News from tg: {}",tg_news);

    lifetime::run_demo();

}

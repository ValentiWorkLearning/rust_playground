pub mod oop_sample;
pub mod blog;
pub mod blog_typed;
use blog::Post;

fn main() {

    oop_sample::run_sample();
    let mut post = Post::new();
    post.add_text("Some cool stuff here");
    post.request_review();
    post.approve();


    let content = post.content();
    println!("post content is {content}");


    let mut post_typed = blog_typed::Post::new();
    
    post_typed.add_text("I ate a salad for lunch today");
    let post_typed = post_typed.request_review();
    let post_typed = post_typed.approve();
    
    assert_eq!(post_typed.content(),"I ate a salad for lunch today")
}


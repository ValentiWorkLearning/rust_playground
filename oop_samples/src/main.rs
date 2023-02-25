pub mod oop_sample;
pub mod blog;
use blog::Post;

fn main() {

    oop_sample::run_sample();
    let mut post = Post::new();
    post.add_text("Some cool stuff here");
    post.request_review();
    post.approve();


    let content = post.content();
    println!("post content is {content}");
}

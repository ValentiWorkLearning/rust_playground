pub mod thread_demo;
pub mod messages_demo;

fn main() {
    thread_demo::thread_demo();
    messages_demo::run_demo();
    
    println!("Hello, world!");
}

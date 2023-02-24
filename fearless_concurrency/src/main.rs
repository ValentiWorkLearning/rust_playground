pub mod thread_demo;
pub mod messages_demo;
pub mod mutex_arc_demo;

fn main() {
    thread_demo::thread_demo();
    messages_demo::run_demo();
    mutex_arc_demo::launch_demo();
    
    println!("Hello, world!");
}

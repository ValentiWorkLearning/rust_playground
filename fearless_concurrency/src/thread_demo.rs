use std::thread;
use std::time::Duration;

fn demo_simple(){
    let first_thread = thread::spawn(
        ||{
            for i in 1..10{
                println!("Running from thread : {i}");
                thread::sleep(Duration::from_millis(1));
            }
        }
    );
    
    first_thread.join().unwrap();

    let second_thread = thread::spawn(
      ||{
        for i in 1..5{
            println!("Running from main thread {i}");
            thread::sleep(Duration::from_millis(2))
        }
      }  
    );
    second_thread.join().unwrap();
}

fn vec_demo(){
    let test_vec = vec![1,2,3,4];
    let handle = thread::spawn(
        move ||{
            for i in test_vec{
                println!("Vec item: {i}")
            }
        }
    );
    handle.join().unwrap();
    
}
pub fn thread_demo(){
    demo_simple();
    vec_demo();
}
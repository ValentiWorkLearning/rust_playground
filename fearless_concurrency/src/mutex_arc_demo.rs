use std::sync::{Arc,Mutex};

use std::thread;

pub fn launch_demo(){
    let counter = Arc::new(Mutex::new(0));
    let value_arc = Arc::clone(&counter);
    let value_arc_second = Arc::clone(&counter);
    let thread_handle_first = thread::spawn(
        move ||{
            let mut value =value_arc.lock().unwrap();
            println!("Current thread is:{:?}", thread::current().id());
            *value = 43;
            
        }
    );
    let thread_handle_second = thread::spawn(
        move ||{            
            let mut value =value_arc_second.lock().unwrap();
            println!("Current thread is:{:?}", thread::current().id());
            *value = 24;
        }
    );
    {   
        let arc_clone = Arc::clone(&counter);
        let result = arc_clone.lock().unwrap();
        println!("Final value is: {:?}",*result);
    }
    thread_handle_first.join().unwrap();
    thread_handle_second.join().unwrap();
}
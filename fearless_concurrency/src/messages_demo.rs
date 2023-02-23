use std::str::FromStr;
use std::sync::mpsc;
use std::{thread, vec};

fn simple_demo(){
    let (tx,rx) = mpsc::channel();
    let handle = thread::spawn(
      move||{
        let val = String::from_str("thread_message");
        tx.send(val).unwrap();
      }  
    );
    let received = rx.recv().unwrap();
    println!("Thread result is: {:?}",received);
    let second_item = rx.try_recv();
    match second_item {
        Ok(received)=>{

        },
        _=>println!("Empty...")
    };
    handle.join().unwrap();
}

fn multiple_wait(){
    let vec_test = vec![1,2,3,4,5];
    let (tx,rx) = mpsc::channel();

    let second_tx = tx.clone();

    let handle = thread::spawn(
      move||{
        for item in vec_test{
            tx.send(item).unwrap();
        }
      }  
    );

    let second_thread = thread::spawn(
        move||{
            let vec_second = vec![5,10,15,20];
            for item in vec_second{
                second_tx.send(item).unwrap();
            }
        }
    );
    for item in &rx{
        println!("Got item: {:?}!",item);
    }
    
    handle.join().unwrap();
    second_thread.join().unwrap();

}
pub fn run_demo(){
    simple_demo();
    multiple_wait();
}
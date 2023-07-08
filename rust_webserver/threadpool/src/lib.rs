use std::{sync::{mpsc,Arc,Mutex},thread};

pub struct ThreadPool{
    workers:Vec<Worker>,
    sender:Option<mpsc::Sender<Job>>,
}

struct Worker{
    id:usize,
    thread_handle: Option<thread::JoinHandle<()>>,
}



type Job = Box<dyn FnOnce() + Send + 'static>;


impl Worker{
    pub fn new(worker_id:usize,receiver: Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{

        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message{
                Ok(job)=>{
                    println!("Worker {worker_id} got a job; executing.");
                    job()
                }
                Err(_)=>{
                    println!("Worker {worker_id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { id: worker_id, thread_handle: Some(thread)}
    }
}
impl ThreadPool{

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    /// 
    pub fn new(num_threads:u8)->ThreadPool{
        assert!(num_threads>0);

        let mut workers = Vec::with_capacity(num_threads as usize);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..num_threads as usize{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        
        ThreadPool{workers,sender:Some(sender)}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }

}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(worker) = worker.thread_handle.take(){
                worker.join().unwrap();
            }
            println!("Shutting down worker {} done!", worker.id);
        }
    }
}
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
pub struct ThreadPoolCreationError<'a> {
    message: &'a str,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is a number of threads in thread pool.
    ///
    /// # Panics
    ///
    /// The `new` will panic if the  size is zero (or less).
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let thread_safe_receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&thread_safe_receiver)));
        }

        ThreadPool { workers, sender }
    }

    // pub fn build(size: usize) -> Result<ThreadPool, ThreadPoolCreationError> {
    //     if size == 0 {
    //        Err(ThreadPoolCreationError{"Cannot create pool without threads"})
    //     }
    //     Ok(ThreadPool)
    // }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}

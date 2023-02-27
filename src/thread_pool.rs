use std::{
    error::Error,
    fmt,
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::thread_pool_errors::{BoxResult, WorkerCreationError};

pub struct ThreadPool {
    //TODO: write benchmark tests for threadpool
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() -> Result<(), Box<dyn Error>> + Send + 'static>;
type ArcReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error creating thread pool")
    }
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<Self, PoolCreationError> {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let sender = Some(sender);
        let receiver: ArcReceiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver)).or(Err(PoolCreationError))?;
            workers.push(worker);
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&mut self, f: F) -> BoxResult<()>
    where
        F: FnOnce() -> Result<(), Box<dyn Error>> + Send + 'static,
    {
        let job: Job = Box::new(f);
        self.sender
            .as_mut()
            .expect("sender should always be some if ThreadPool is not getting dropped")
            .send(job)?;

        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Shutting down thread pool");
        drop(self.sender.take());
        for worker in self.workers.iter_mut() {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: ArcReceiver) -> Result<Self, WorkerCreationError> {
        let thread_builder = thread::Builder::new();
        let thread = thread_builder.spawn(move || loop {
            let job = match receiver.lock().unwrap().recv() {
                Err(_) => break,
                Ok(job) => job,
            };

            println!("Worker {id} got a job, start execution");

            job().unwrap_or_else(|err| eprintln!("Worker {id} got an error: {err}"));
        });

        let thread = Some(thread?);
        Ok(Worker { id, thread })
    }
}

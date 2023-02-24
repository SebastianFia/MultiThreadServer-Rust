use std::{
    thread::{self, JoinHandle},
    sync::{Arc, Mutex, mpsc},
};

type Job = Box<dyn FnOnce() + Send + 'static>;
type SendableReceiver = Arc<Mutex<mpsc::Receiver<Job>>>; 

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: SendableReceiver) -> Self  {
        let thread = Some(thread::spawn(move || loop {
            let job = match receiver.lock().unwrap().recv() {
                Err(_) => {
                    break    
                }
                Ok(job) => job,
            };

            println!("Worker {} got a job, start execution", id);

            job();
        }));

        Worker{id, thread}
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let sender = Some(sender);
        let receiver: SendableReceiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool{workers, sender}
    }

    pub fn execute<F>(&mut self, f: F) 
    where F: FnOnce() + Send + 'static 
    {
       let job: Job = Box::new(f);
       self.sender.as_mut().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.iter_mut() {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        };
    }
}
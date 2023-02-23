use std::{
    thread::{self, JoinHandle},
    sync::{Arc, Mutex, mpsc},
};

type Job = Box<dyn FnOnce() + Send + 'static>;
type SendableReceiver = Arc<Mutex<mpsc::Receiver<Job>>>; 

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: SendableReceiver) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job, start execution", id);

            job();
        });

        Worker{id, thread}
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
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
       self.sender.send(job).unwrap();
    }
}
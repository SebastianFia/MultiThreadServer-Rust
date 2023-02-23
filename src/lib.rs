use std::{
    thread::{self, JoinHandle},
    //sync::{Arc, Mutex, mpsc},
};

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
}


impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool{threads: Vec::with_capacity(size)}
    }

    pub fn execute<F>(&mut self, f: F) 
    where F: FnOnce() + Send + 'static 
    {
        assert!(self.threads.len() < self.threads.capacity());

        let handle = thread::spawn(f);
        self.threads.push(handle);
        
        println!("New thread created:\n{:#?}", self.threads);
    }
}
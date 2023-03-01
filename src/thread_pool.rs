use std::{
    error::Error,
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::thread_pool_errors::{BoxResult, PoolCreationError, WorkerCreationError};

pub struct ThreadPool {
    //TODO: write unit tests for threadpool
    //TODO: write benchmark tests for threadpool
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() -> Result<(), Box<dyn Error>> + Send + 'static>;
type ArcReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;

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

    pub fn size(&self) -> usize {
        self.workers.len()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_10_thread_pool() {
        let counter = Arc::new(Mutex::new(0));

        {
            let mut pool = ThreadPool::build(10).unwrap();
            for _ in 0..10 {
                let counter_clone = Arc::clone(&counter);
                pool.execute(move || {
                    thread::sleep(std::time::Duration::from_millis(10));
                    let mut counter_guard = counter_clone.lock().unwrap();
                    *counter_guard += 1;
                    Ok(())
                })
                .unwrap();
            }
        }

        assert_eq!(counter.lock().unwrap().to_owned(), 10);
    }

    #[test]
    fn message_passing_thread_pool() {
        let num_channels = 10;
        let num_threads = num_channels - 1;
        let (senders, receivers) = get_arc_senders_and_receivers(num_channels);
        let mut pool = ThreadPool::build(num_threads).unwrap();
        send_first_message(1, &senders);
        pass_message_through_pool(&mut pool, &senders, &receivers);
        let last_msg_val = get_last_message_val(num_channels, &receivers);

        assert_eq!(last_msg_val, num_channels as i32);
    }

    type SendersVec = Vec<Arc<Mutex<mpsc::Sender<i32>>>>;
    type ReceiversVec = Vec<Arc<Mutex<mpsc::Receiver<i32>>>>;

    fn send_first_message(message: i32, senders: &SendersVec) {
        senders[0].lock().unwrap().send(message).unwrap();
    }

    fn get_last_message_val(num_channels: usize, receivers: &ReceiversVec) -> i32 {
        let receiver = Arc::clone(&receivers[num_channels - 1]);
        let last_message_val = receiver.lock().unwrap().recv().unwrap();
        last_message_val
    }

    fn get_arc_senders_and_receivers(num_channels: usize) -> (SendersVec, ReceiversVec) {
        let mut senders = Vec::with_capacity(num_channels);
        let mut receivers = Vec::with_capacity(num_channels);
        for _ in 0..num_channels {
            let (sender, receiver) = mpsc::channel::<i32>();
            senders.push(Arc::new(Mutex::new(sender)));
            receivers.push(Arc::new(Mutex::new(receiver)));
        }
        (senders, receivers)
    }

    fn pass_message_through_pool(
        pool: &mut ThreadPool,
        senders: &SendersVec,
        receivers: &ReceiversVec,
    ) {
        for i in 0..pool.size() {
            let receiver = Arc::clone(&receivers[i]);
            let sender = Arc::clone(&senders[ i + 1]);
            pool.execute(move || {
                let received = receiver.lock().unwrap().recv().unwrap();
                sender.lock().unwrap().send(received + 1).unwrap(); 
                Ok(())
            })
            .unwrap();
        }
    }
}

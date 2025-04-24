
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // TODO: Create a channel for sending jobs
        let (sender, receiver) = mpsc::channel();
        // Wrap the receiver in Arc<Mutex<...>> to share it among workers
        let receiver = Arc::new(Mutex::new(receiver));
       
        // TODO: Create and store workers
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // Create a new worker and pass the receiver
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        // TODO: Return the ThreadPool
        // Return the ThreadPool with workers and sender
        ThreadPool {
            workers,
            sender,
        }
        
    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Create a job from the closure and send it to a worker
        let job = Box::new(f);
        // Send the job to the channel
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: Send terminate message to all workers
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        
        // TODO: Wait for all workers to finish
        for worker in &mut self.workers {
            // Take the thread handle and join it
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
        println!("ThreadPool has been shut down.");
        
    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // TODO: Create a thread that loops and receives jobs from the channel
        let thread = thread::spawn(move || {
            loop {
                // Lock the receiver and wait for a message
                let message = receiver.lock().unwrap().recv().unwrap();
                
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job(); // Execute the job
                    }
                    Message::Terminate => {
                        println!("Worker {} terminating.", id);
                        break; // Exit the loop on terminate message
                    }
                }
            }
        });
        
        // TODO: Return the Worker
        Worker {
            id,
            thread: Some(thread), // Store the thread handle
        }
        
    }
}

fn main() {
    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    
    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }
    
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
}
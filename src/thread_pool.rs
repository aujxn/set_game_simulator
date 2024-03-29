/* This module implements a thread pool so work can be
 * distributed to multiple cores. The threadpool instance
 * takes a closure and the creating thread holds until
 * all the work is complete when it goes out of scope
 */
use ::std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/* provides a way to move a closure out of a smart pointer */
pub trait FnBox {
    fn call_box(self: Box<Self>);
}

/* calls the closure in the smart pointer */
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

/* wraps jobs for the thread pool to complete */
pub enum Message {
    NewJob(Job),
    Kill,
}

/* manages a thread in the pool */
struct Worker {
    id: usize, //for debugging purposes
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = rx.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    *job.call_box();
                }
                Message::Kill => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/* manages the workers of the pool */
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

/* when ThreadPool goes out of scope the creating thread
 * now holds until all workers are completed
 */
impl Drop for ThreadPool {
    fn drop(&mut self) {
        /* send kill messages */
        for _ in &mut self.workers {
            self.sender.send(Message::Kill).unwrap();
        }

        /* hold until all workers are completed */
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /* constructor - size is number of threads desired */
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        /* uses multiple transmitter - single receiver to send work */
        let (tx, rx) = mpsc::channel();

        /* channel is single reciever so needs arc/mutex to share between workers */
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, rx.clone()));
        }

        ThreadPool {
            workers,
            sender: tx,
        }
    }

    /* sends work to the workers */
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

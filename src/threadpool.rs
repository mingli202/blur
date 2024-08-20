use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    tx: Option<mpsc::Sender<Job>>,
    threads: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(n_workers: usize) -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut handles = Vec::with_capacity(n_workers);

        for _ in 0..n_workers {
            handles.push(Worker::new(Arc::clone(&rx)));
        }

        ThreadPool {
            tx: Some(tx),
            threads: handles,
        }
    }

    pub fn execute(&self, f: Job) {
        if let Some(tx) = &self.tx {
            tx.send(f).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.tx.take());

        for worker in &mut self.threads {
            if let Some(handle) = worker.thread.take() {
                handle.join().unwrap();
            }
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(rx: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let msg = rx.lock().unwrap().recv();

            match msg {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });

        Worker {
            thread: Some(handle),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let mut workers = Vec::with_capacity(size);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&rx)));
        }

        Self {
            workers,
            tx: Some(tx),
        }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.tx
            .as_ref()
            .unwrap()
            .send(Box::new(f))
            .expect("Failed to execute the function");
    }

    pub fn join_all(mut self) {
        drop(self.tx.take().unwrap());
        self.workers.into_iter().for_each(|worker| worker.join());
    }
}

struct Worker {
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        Self {
            handle: Some(thread::spawn(move || loop {
                let Ok(job) = rx.lock().unwrap().recv() else {
                    break;
                };

                job();
            })),
        }
    }

    pub fn join(mut self) {
        self.handle
            .take()
            .unwrap()
            .join()
            .expect("Failed to join worker thread");
    }
}

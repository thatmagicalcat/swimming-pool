use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use super::threadpool::Job;

pub(crate) struct Worker {
    handle: Option<thread::JoinHandle<()>>,
    working: Arc<Mutex<bool>>,
}

impl Worker {
    pub fn init(&mut self, rx: Arc<Mutex<mpsc::Receiver<Job>>>) {
        assert!(self.handle.is_none(), "Worker is already initialized");

        let working = Arc::clone(&self.working);
        self.handle = {
            Some(thread::spawn(move || loop {
                *working.lock().unwrap() = false;
                let Ok(job) = rx.lock().unwrap().recv() else {
                    break;
                };

                *working.lock().unwrap() = true;
                job();
            }))
        };
    }

    pub fn join(mut self) {
        self.handle
            .take()
            .unwrap()
            .join()
            .expect("Failed to join worker thread");
    }

    pub fn is_working(&self) -> bool {
        *self.working.lock().unwrap()
    }
}

impl Default for Worker {
    fn default() -> Self {
        Self {
            handle: None,
            working: Arc::new(Mutex::new(false)),
        }
    }
}

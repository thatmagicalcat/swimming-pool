use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use super::threadpool::Job;

pub(crate) struct Worker {
    handle: Option<thread::JoinHandle<()>>,
    working: Arc<Mutex<bool>>,
}

impl Worker {
    pub fn new(rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let working = Arc::new(Mutex::new(false));

        Self {
            working: Arc::clone(&working),
            handle: {
                Some(thread::spawn(move || loop {
                    *working.lock().unwrap() = false;
                    let Ok(job) = rx.lock().unwrap().recv() else {
                        break;
                    };

                    *working.lock().unwrap() = true;
                    job();
                }))
            },
        }
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

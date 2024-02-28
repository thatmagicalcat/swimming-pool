use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use crate::worker::Worker;

pub(crate) type Job = Box<dyn FnOnce() + Send + 'static>;

/// Thread pool struct.
///
/// # Panics
/// Panics when the generic parameter (POOL_SIZE)
/// is zero.
///
/// # Example
/// ```rust
/// use std::thread;
/// use std::time;
///
/// use swimming_pool::ThreadPool;
///
/// fn main() {
///     // Initialize a thread pool with 5 threads
///     let pool = ThreadPool::new(5);
///
///     pool.execute(|| {
///         // Send job to the pool
///         thread::sleep(time::Duration::from_secs(5));
///     });
///
///     pool.join_all();
/// }
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Initialize a new thread pool
    ///
    /// # Panics
    /// Panics when the pool size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size != 0, "Minimum of one thread is required in the pool.");

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&rx)));
        }

        Self {
            workers,
            tx: Some(tx),
        }
    }

    /// Send a job to the pool.
    /// This job will be picked and executed
    /// by any free worker thread.
    ///
    /// # Example
    /// ```rust
    /// use threadpool::Threadpool;
    ///
    /// let pool = ThreadPool::new(10);
    /// pool.execute(|| {
    ///     // Send job to the pool
    /// });
    ///```
    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.tx
            .as_ref()
            .unwrap()
            .send(Box::new(f))
            .expect("Failed to execute the function");
    }

    /// Join all the worker threads (wait for
    /// them to finish executing their job).
    pub fn join_all(mut self) {
        drop(self.tx.take().unwrap());
        self.workers.into_iter().for_each(|worker| worker.join());
    }

    /// Returns the number of worker threads in the
    /// thread pool.
    pub fn get_pool_size(&self) -> usize {
        self.workers.len()
    }

    /// Returns the number of worker threads
    /// which are currently executing a job.
    pub fn get_working_threads(&self) -> usize {
        self.workers
            .iter()
            .filter(|worker| worker.is_working())
            .count()
    }
}

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use super::worker::Worker;

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
///     // Initialize a thread pool with 10 threads
///     let pool = ThreadPool::<10>::new();
///
///     pool.execute(|| {
///         // Send job to the pool
///         thread::sleep(time::Duration::from_secs(5));
///     });
///
///     pool.join_all();
/// }
/// ```
pub struct ThreadPool<const POOL_SIZE: usize> {
    workers: [Worker; POOL_SIZE],
    tx: Option<mpsc::Sender<Job>>,
}

impl<const POOL_SIZE: usize> ThreadPool<POOL_SIZE> {
    /// Initialize a new thread pool
    ///
    /// # Panics
    /// Panics when the generic parameter (POOL_SIZE)
    /// is zero.
    pub fn new() -> Self {
        assert!(
            POOL_SIZE != 0,
            "Minimum of one thread is required in the pool."
        );

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = std::array::from_fn(|_| Worker::default());
        workers
            .iter_mut()
            .for_each(|worker| worker.init(Arc::clone(&rx)));

        Self {
            workers,
            tx: Some(tx),
        }
    }

    /// Send a job to the pool.
    /// This job will be picked and executed
    /// by any free woker thread.
    ///
    /// # Example
    /// ```rust
    /// use threadpool::Threadpool;
    ///
    /// let pool = ThreadPool::<10>::new();
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
        POOL_SIZE
    }

    /// Returns the number of woker threads
    /// which are currently executing a job.
    pub fn get_working_threads(&self) -> usize {
        self.workers
            .iter()
            .filter(|worker| worker.is_working())
            .count()
    }
}

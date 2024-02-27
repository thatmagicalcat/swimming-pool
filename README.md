# Swimming pool
A simple threadpool for running a number of jobs on a fixed number of threads.

## Example usage
Add it to your project
```sh
$ cargo add swimming-pool
```

Example code
```rust
use std::thread;
use std::time::Duration;

use swimming_pool::ThreadPool;

fn main() {
    // Create a thread pool with 5 worker threads
    let pool = ThreadPool::new(5);

    // Spawn 5 jobs
    for _ in 0..5 {
        pool.execute(|| {
            thread::sleep(Duration::from_secs(5));
            println!("Printing after 5 seconds");
        })
    }

    pool.join_all();
}
```

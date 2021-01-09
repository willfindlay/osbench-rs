use std::thread;

use crate::{Error, Result};

const NUM_THREADS: usize = 100;

/// Creates and joins `[NUM_THREADS]` threads which immediately exit.
pub fn create_threads() -> Result<()> {
    let mut handles = vec![];

    // Spawn threads
    for _ in 0..handles.len() {
        handles.push(match thread::Builder::new().spawn(|| {}) {
            Ok(handle) => handle,
            Err(e) => return Err(Error::ThreadSpawnError(format!("{}", e))),
        });
    }

    for handle in handles {
        match handle.join() {
            Ok(_) => {}
            Err(e) => return Err(Error::ThreadJoinError(format!("{:?}", e))),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_threads_test() {
        create_threads().expect("Failed to create processes");
    }
}

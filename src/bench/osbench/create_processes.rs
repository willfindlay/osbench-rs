use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult, Pid};
use std::process::exit;

use crate::{Error, Result};

const NUM_PROCESSES: usize = 100;

/// Creates and reaps `[NUM_PROCESSES]` processes which immediately exit.
pub fn create_processes() -> Result<()> {
    let pids = &mut [Pid::from_raw(0); NUM_PROCESSES];

    // Spawn processes
    for i in 0..pids.len() {
        pids[i] = match spawn_process() {
            Ok(pid) => pid,
            Err(e) => return Err(e),
        }
    }

    // Reap processes
    for &mut pid in pids {
        match waitpid(pid, None) {
            Ok(_) => {}
            Err(e) => return Err(Error::ProcessWaitError(format!("{}", e))),
        }
    }

    Ok(())
}

/// Spawn a process using `fork(2)`.
fn spawn_process() -> Result<Pid> {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => Ok(child),
        Ok(ForkResult::Child) => {
            exit(0);
        }
        Err(e) => Err(Error::ProcessSpawnError(format!("{}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_processes_test() {
        create_processes().expect("Failed to create processes");
    }
}

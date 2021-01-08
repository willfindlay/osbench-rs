use std::result;

use thiserror::Error;

/// Canonical error type for this crate.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to create file: {0}")]
    FileCreationError(String),
    #[error("Failed to delete file: {0}")]
    FileDeletionError(String),
    #[error("Failed to spawn a new process: {0}")]
    ProcessSpawnError(String),
    #[error("Failed to wait for process: {0}")]
    ProcessWaitError(String),
}

pub type Result<T> = result::Result<T, Error>;

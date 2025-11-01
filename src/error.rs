use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PnumError {
    #[error("Dir read error: {0}")]
    DirReadError(#[from] std::io::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
}

pub type PnumResult<T> = Result<T, PnumError>;

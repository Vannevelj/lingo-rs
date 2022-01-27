use thiserror::Error;

#[derive(Error, Debug)]
pub enum LingoError {
    #[error("Could not read path")]
    InvalidPath,
}

pub type LingoResult<T> = Result<T, LingoError>;
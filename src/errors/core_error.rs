use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("executable can't be read")]
    NoBinaryPath,

    #[error("task does not specified")]
    NoTask,

    #[error("combos path does not specified")]
    NoCombos,

    #[error("task does not implemented")]
    TaskNotImplemented,

    #[error("no results")]
    NoResults,

    #[error("unexpected error with args")]
    UnexpectedArgs,

    #[error(transparent)]
    IoError(#[from] io::Error),
    /*#[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,*/
}

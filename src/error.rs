use std::{
    cell::{BorrowError, BorrowMutError},
    string::FromUtf8Error,
};
use thiserror::Error;

pub type Result<T> = anyhow::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Anyhow {0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("Invalid regex {0}")]
    Regex(#[from] regex::Error),

    #[error("Format Error {0}")]
    Format(#[from] std::fmt::Error),

    #[error("UTF-8 Error {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("Borrow error {0}")]
    Borrow(#[from] BorrowError),

    #[error("Borrow error {0}")]
    BorrowMut(#[from] BorrowMutError),

    #[error("Object not found with identity {0}")]
    ObjectNotFound(String),

    #[error("Invalid label string")]
    InvalidLabelString,
}

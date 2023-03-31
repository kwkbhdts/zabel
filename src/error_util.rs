//! Error utility

use std::io;

/// Type alias of Result
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// This function creates an error instance.
/// 
/// # Arguments
/// * message : A message about error.
#[allow(dead_code)]
pub fn create_error<S: AsRef<str>>(messge: S) -> io::Error {
    io::Error::new(io::ErrorKind::Other, messge.as_ref())
}
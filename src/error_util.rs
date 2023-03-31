//! Error utility

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

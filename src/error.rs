use std::fmt;

#[derive(Debug)]
pub enum TldrError {
    PageNotFound(String),
    CacheNotFound,
    NetworkError(String),
    FileNotFound(String),
    NoArguments,
}

impl fmt::Display for TldrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TldrError::PageNotFound(name) => write!(f, "page not found: {name}"),
            TldrError::CacheNotFound => write!(f, "cache not found"),
            TldrError::NetworkError(msg) => write!(f, "network error: {msg}"),
            TldrError::FileNotFound(path) => write!(f, "file not found: {path}"),
            TldrError::NoArguments => write!(f, "no arguments given"),
        }
    }
}

impl std::error::Error for TldrError {}

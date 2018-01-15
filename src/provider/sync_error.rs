use std::error;
// use reqwest;
use std::fmt;
use std::sync;

#[derive(Debug)]
pub enum SyncError {
    ConfigurationError,
    // HttpError(reqwest::Error),
    LibraryAccessError//(sync::PoisonError<sync::MutexGuard<'_, Library>>)
}

impl<T> From<sync::PoisonError<T>> for SyncError {
    fn from(_: sync::PoisonError<T>) -> SyncError {
        SyncError::LibraryAccessError
    }
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SyncError::ConfigurationError => write!(f, "Configuration Error"),
            // SyncError::HttpError(ref err) => write!(f, "HTTP Error: {}", err),
            SyncError::LibraryAccessError => write!(f, "Library Access Error")
        }
    }
}

impl error::Error for SyncError {
    fn description(&self) -> &str {
        match *self {
            SyncError::ConfigurationError => "invalid provider configuration",
            // SyncError::HttpError(ref err) => err.description(),
            SyncError::LibraryAccessError => "a thread died while locking the library mutex"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // SyncError::HttpError(ref error) => Some(error),
            _ => None
        }
    }
}
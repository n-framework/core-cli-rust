use std::error::Error;
use std::fmt::{Display, Formatter};

/// Represents the category of a logging error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoggingErrorKind {
    /// An underlying I/O error occurred.
    Io,
    /// An unexpected internal system error occurred.
    Internal,
}

/// A specialized error type for logging operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoggingError {
    /// The category of the error.
    kind: LoggingErrorKind,
    /// A human-readable message describing the error.
    message: String,
}

impl LoggingError {
    /// Creates a new I/O-related logging error.
    pub fn io(message: impl Into<String>) -> Self {
        Self {
            kind: LoggingErrorKind::Io,
            message: message.into(),
        }
    }

    /// Creates a new internal logging error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            kind: LoggingErrorKind::Internal,
            message: message.into(),
        }
    }

    /// Returns the human-readable error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the error category.
    pub fn kind(&self) -> LoggingErrorKind {
        self.kind
    }
}

impl Display for LoggingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LoggingError {}

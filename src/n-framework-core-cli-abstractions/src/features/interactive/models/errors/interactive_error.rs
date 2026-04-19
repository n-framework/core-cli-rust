use std::error::Error;
use std::fmt::{Display, Formatter};

/// Represents the category of an interactive error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractiveErrorKind {
    /// Operation was cancelled by the user.
    Cancelled,
    /// An underlying I/O error occurred.
    Io,
    /// User input or configuration failed validation.
    Validation,
    /// An unexpected internal system error occurred.
    Internal,
}

/// A unified error type for all interactive CLI operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractiveError {
    /// The category of the error.
    kind: InteractiveErrorKind,
    /// A human-readable message describing the error.
    message: String,
}

impl InteractiveError {
    pub fn cancelled(message: impl Into<String>) -> Self {
        Self {
            kind: InteractiveErrorKind::Cancelled,
            message: message.into(),
        }
    }

    pub fn io(message: impl Into<String>) -> Self {
        Self {
            kind: InteractiveErrorKind::Io,
            message: message.into(),
        }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self {
            kind: InteractiveErrorKind::Validation,
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            kind: InteractiveErrorKind::Internal,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> &InteractiveErrorKind {
        &self.kind
    }

    pub fn is_cancelled(&self) -> bool {
        self.kind == InteractiveErrorKind::Cancelled
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for InteractiveError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for InteractiveError {}

#[cfg(test)]
#[path = "interactive_error.tests.rs"]
mod tests;

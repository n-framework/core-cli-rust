use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptErrorKind {
    Cancelled,
    Io,
    Validation,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptError {
    kind: PromptErrorKind,
    message: String,
}

impl PromptError {
    pub fn cancelled(message: impl Into<String>) -> Self {
        Self {
            kind: PromptErrorKind::Cancelled,
            message: message.into(),
        }
    }

    pub fn io(message: impl Into<String>) -> Self {
        Self {
            kind: PromptErrorKind::Io,
            message: message.into(),
        }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self {
            kind: PromptErrorKind::Validation,
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            kind: PromptErrorKind::Internal,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> &PromptErrorKind {
        &self.kind
    }

    pub fn is_cancelled(&self) -> bool {
        self.kind == PromptErrorKind::Cancelled
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for PromptError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for PromptError {}

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliAdapterErrorKind {
    Help,
    Parse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliAdapterError {
    kind: CliAdapterErrorKind,
    message: String,
}

impl CliAdapterError {
    pub fn help(message: String) -> Self {
        Self {
            kind: CliAdapterErrorKind::Help,
            message,
        }
    }

    pub fn parse(message: String) -> Self {
        Self {
            kind: CliAdapterErrorKind::Parse,
            message,
        }
    }

    pub fn is_help(&self) -> bool {
        self.kind == CliAdapterErrorKind::Help
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for CliAdapterError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for CliAdapterError {}

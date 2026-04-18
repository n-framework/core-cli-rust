use crate::features::interactive::models::errors::interactive_error::InteractiveError;
use crate::features::logging::abstractions::spinner::Spinner;

pub trait Logger: Send + Sync {
    /// Prints an intro block.
    fn intro(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints an outro block.
    fn outro(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints a cancellation message.
    fn cancel_log(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints a simple informational message.
    fn log_info(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints a success message.
    fn log_success(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints a warning message.
    fn log_warning(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints an error message.
    fn log_error(&self, message: &str) -> Result<(), InteractiveError>;

    /// Starts a spinner, returning a handle to update or stop it.
    fn spinner(&self, message: &str) -> Result<Box<dyn Spinner>, InteractiveError>;
}

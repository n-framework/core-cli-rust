use crate::features::interactive::models::errors::interactive_error::InteractiveError;
use crate::features::logging::abstractions::spinner::Spinner;

pub trait Logger: Send + Sync {
    /// Prints an intro block to mark the beginning of a related set of operations.
    /// Typically used at the start of a command execution.
    fn intro(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints an outro block to mark the successful completion of a set of operations.
    /// Typically used at the end of a command execution.
    fn outro(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints a cancellation message to mark that operations were aborted.
    fn cancel_log(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints a simple informational message.
    fn log_info(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints a success message (typically with a checkmark).
    fn log_success(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints a warning message (typically in yellow).
    fn log_warning(&self, message: &str) -> Result<(), InteractiveError>;

    /// Prints an error message (typically in red).
    fn log_error(&self, message: &str) -> Result<(), InteractiveError>;

    /// Starts a spinner to indicate a long-running background task.
    /// Returns a handle to update text or stop the spinner.
    fn spinner(&self, message: &str) -> Result<Box<dyn Spinner>, InteractiveError>;
}

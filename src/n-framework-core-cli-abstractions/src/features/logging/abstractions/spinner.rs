pub trait Spinner: Send + Sync {
    /// Completes the spinner with a default success state.
    fn stop(&self, message: &str);

    /// Completes the spinner with an explicit success mark.
    fn success(&self, message: &str);

    /// Completes the spinner with an explicit error mark.
    fn error(&self, message: &str);

    /// Cancels the spinner (typically leaves it in a warning or default state).
    fn cancel_log(&self, message: &str);

    /// Updates the text of the spinner without completing it.
    fn set_message(&self, message: &str);

    /// Returns true if the spinner has been completed (success, error, or stopped).
    fn is_finished(&self) -> bool;
}

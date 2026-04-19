use super::cliclack_prompt_service::CliclackPromptService;
use super::cliclack_spinner::CliclackSpinner;
use n_framework_core_cli_abstractions::{Logger, LoggingError, Spinner};
use std::sync::Mutex;

impl Logger for CliclackPromptService {
    fn intro(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::intro(message).map_err(|e| LoggingError::io(e.to_string()))
    }

    fn outro(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::outro(message).map_err(|e| LoggingError::io(e.to_string()))
    }

    fn log_cancel(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::outro_cancel(message).map_err(|e| LoggingError::io(e.to_string()))
    }

    fn log_info(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::log::info(message).map_err(|e| LoggingError::io(e.to_string()))
    }

    fn log_success(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::log::success(message).map_err(|e| LoggingError::io(e.to_string()))
    }

    fn log_warning(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::log::warning(message).map_err(|e| LoggingError::io(e.to_string()))
    }

    fn log_error(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::log::error(message).map_err(|e| LoggingError::io(e.to_string()))
    }

    fn spinner(&self, message: &str) -> Result<Box<dyn Spinner>, LoggingError> {
        let pb = cliclack::spinner();
        pb.start(message);
        Ok(Box::new(CliclackSpinner {
            inner: Mutex::new(Some(pb)),
        }))
    }
}

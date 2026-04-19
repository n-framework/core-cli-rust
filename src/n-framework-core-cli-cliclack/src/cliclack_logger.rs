use super::cliclack_prompt_service::CliclackPromptService;
use super::cliclack_spinner::CliclackSpinner;
use n_framework_core_cli_abstractions::{Logger, LoggingError, Spinner};
use std::sync::Mutex;

impl Logger for CliclackPromptService {
    fn intro(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::intro(pad_multiline(message, "│  ")).map_err(|e| LoggingError::io(e.to_string()))
    }

    fn outro(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::outro(pad_multiline(message, "   ")).map_err(|e| LoggingError::io(e.to_string()))
    }

    fn log_cancel(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::outro_cancel(pad_multiline(message, "   "))
            .map_err(|e| LoggingError::io(e.to_string()))
    }

    fn log_info(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::log::info(pad_multiline(message, "│  "))
            .map_err(|e| LoggingError::io(e.to_string()))
    }

    fn log_step(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::log::step(pad_multiline(message, "│  "))
            .map_err(|e| LoggingError::io(e.to_string()))
    }

    fn log_success(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::log::success(pad_multiline(message, "│  "))
            .map_err(|e| LoggingError::io(e.to_string()))
    }

    fn log_warning(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::log::warning(pad_multiline(message, "│  "))
            .map_err(|e| LoggingError::io(e.to_string()))
    }

    fn log_error(&self, message: &str) -> Result<(), LoggingError> {
        cliclack::log::error(pad_multiline(message, "│  "))
            .map_err(|e| LoggingError::io(e.to_string()))
    }

    fn spinner(&self, message: &str) -> Result<Box<dyn Spinner>, LoggingError> {
        let pb = cliclack::spinner();
        pb.start(pad_multiline(message, "│  "));
        Ok(Box::new(CliclackSpinner {
            inner: Mutex::new(Some(pb)),
        }))
    }
}

fn pad_multiline(message: &str, padding: &str) -> String {
    if !message.contains('\n') {
        return message.to_string();
    }
    let mut padded = String::new();
    for (i, line) in message.lines().enumerate() {
        if i > 0 {
            padded.push('\n');
            padded.push_str(padding);
        }
        padded.push_str(line);
    }
    padded
}

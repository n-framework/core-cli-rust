use super::inquirer_prompt_service::InquirerPromptService;
use super::inquirer_spinner::InquirerConsoleSpinner;
use n_framework_core_cli_abstractions::{Logger, LoggingError, Spinner};
use std::sync::RwLock;
use std::sync::atomic::AtomicBool;

impl Logger for InquirerPromptService {
    fn intro(&self, message: &str) -> Result<(), LoggingError> {
        println!("{}", pad_multiline(message, ""));
        Ok(())
    }

    fn outro(&self, message: &str) -> Result<(), LoggingError> {
        println!("{}", pad_multiline(message, ""));
        Ok(())
    }

    fn log_cancel(&self, message: &str) -> Result<(), LoggingError> {
        println!("{}", pad_multiline(message, ""));
        Ok(())
    }

    fn log_info(&self, message: &str) -> Result<(), LoggingError> {
        println!("INFO: {}", pad_multiline(message, "      "));
        Ok(())
    }

    fn log_step(&self, message: &str) -> Result<(), LoggingError> {
        println!("STEP: {}", pad_multiline(message, "      "));
        Ok(())
    }

    fn log_success(&self, message: &str) -> Result<(), LoggingError> {
        println!("SUCCESS: {}", pad_multiline(message, "         "));
        Ok(())
    }

    fn log_warning(&self, message: &str) -> Result<(), LoggingError> {
        println!("WARNING: {}", pad_multiline(message, "         "));
        Ok(())
    }

    fn log_error(&self, message: &str) -> Result<(), LoggingError> {
        println!("ERROR: {}", pad_multiline(message, "       "));
        Ok(())
    }

    fn spinner(&self, message: &str) -> Result<Box<dyn Spinner>, LoggingError> {
        println!("... {}", pad_multiline(message, "    "));
        Ok(Box::new(InquirerConsoleSpinner {
            message: RwLock::new(message.to_string()),
            finished: AtomicBool::new(false),
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

use std::sync::atomic::AtomicBool;
use std::sync::RwLock;
use n_framework_core_cli_abstractions::{Logger, LoggingError, Spinner};
use super::inquirer_prompt_service::InquirerPromptService;
use super::inquirer_spinner::InquirerConsoleSpinner;

impl Logger for InquirerPromptService {
    fn intro(&self, message: &str) -> Result<(), LoggingError> {
        println!("{}", message);
        Ok(())
    }

    fn outro(&self, message: &str) -> Result<(), LoggingError> {
        println!("{}", message);
        Ok(())
    }

    fn log_cancel(&self, message: &str) -> Result<(), LoggingError> {
        println!("{}", message);
        Ok(())
    }

    fn log_info(&self, message: &str) -> Result<(), LoggingError> {
        println!("INFO: {}", message);
        Ok(())
    }

    fn log_success(&self, message: &str) -> Result<(), LoggingError> {
        println!("SUCCESS: {}", message);
        Ok(())
    }

    fn log_warning(&self, message: &str) -> Result<(), LoggingError> {
        println!("WARNING: {}", message);
        Ok(())
    }

    fn log_error(&self, message: &str) -> Result<(), LoggingError> {
        println!("ERROR: {}", message);
        Ok(())
    }

    fn spinner(&self, message: &str) -> Result<Box<dyn Spinner>, LoggingError> {
        println!("... {}", message);
        Ok(Box::new(InquirerConsoleSpinner {
            message: RwLock::new(message.to_string()),
            finished: AtomicBool::new(false),
        }))
    }
}

use std::io::{self, IsTerminal};
use std::sync::Mutex;

use n_framework_core_cli_abstractions::{
    InteractiveError, InteractivePrompt, Logger, SelectOption, Spinner,
};

pub struct CliclackSpinner {
    inner: Mutex<Option<cliclack::ProgressBar>>,
}

impl Spinner for CliclackSpinner {
    fn set_message(&self, _message: &str) {
        // Not directly supported on cliclack ProgressBar
    }

    fn success(&self, message: &str) {
        if let Some(pb) = self.inner.lock().unwrap().take() {
            pb.stop(message);
        }
    }

    fn error(&self, message: &str) {
        if let Some(pb) = self.inner.lock().unwrap().take() {
            pb.error(message);
        }
    }

    fn cancel_log(&self, message: &str) {
        if let Some(pb) = self.inner.lock().unwrap().take() {
            pb.cancel(message);
        }
    }

    fn stop(&self, message: &str) {
        if let Some(pb) = self.inner.lock().unwrap().take() {
            pb.stop(message);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CliclackPromptService;

impl CliclackPromptService {
    pub fn new() -> Self {
        Self
    }

    fn map_cliclack_error(error: std::io::Error, context: &str) -> InteractiveError {
        // Cliclack functions just return std::io::Error
        // In cliclack, cancel uses std::io::ErrorKind::Interrupted.
        if error.kind() == std::io::ErrorKind::Interrupted {
            InteractiveError::cancelled(context)
        } else {
            InteractiveError::io(format!("{}: {}", context, error))
        }
    }
}

impl Default for CliclackPromptService {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractivePrompt for CliclackPromptService {
    fn is_interactive(&self) -> bool {
        #[cfg(test)]
        if std::env::var("NFW_TEST_FORCE_INTERACTION").is_err() {
            return false;
        }

        if std::env::var("CI").is_ok() || std::env::var("TERM").is_ok_and(|v| v == "dumb") {
            return false;
        }

        io::stdin().is_terminal() && io::stdout().is_terminal()
    }

    fn text(&self, message: &str, default: Option<&str>) -> Result<String, InteractiveError> {
        let mut prompt = cliclack::input(message);
        if let Some(default_value) = default {
            prompt = prompt.default_input(default_value);
        }

        prompt
            .interact()
            .map_err(|e| Self::map_cliclack_error(e, "prompt failed"))
    }

    fn confirm(&self, message: &str, default: bool) -> Result<bool, InteractiveError> {
        cliclack::confirm(message)
            .initial_value(default)
            .interact()
            .map_err(|e| Self::map_cliclack_error(e, "confirmation failed"))
    }

    fn password(&self, message: &str) -> Result<String, InteractiveError> {
        cliclack::password(message)
            .interact()
            .map_err(|e| Self::map_cliclack_error(e, "password prompt failed"))
    }

    fn select(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<SelectOption, InteractiveError> {
        let mut prompt = cliclack::select(message);
        for opt in options.iter() {
            let label = if let Some(desc) = opt.description() {
                format!("{} - {}", opt.label(), desc)
            } else {
                opt.label().to_string()
            };
            prompt = prompt.item(opt.clone(), label, "");
        }

        if let Some(opt) = default_index.and_then(|idx| options.get(idx)) {
            prompt = prompt.initial_value(opt.clone());
        }

        prompt
            .interact()
            .map_err(|e| Self::map_cliclack_error(e, "selection failed"))
    }

    fn select_index(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<usize, InteractiveError> {
        let selected = self.select(message, options, default_index)?;
        options
            .iter()
            .position(|opt| opt == &selected)
            .ok_or_else(|| InteractiveError::internal("selected option not found in list"))
    }

    fn multiselect(
        &self,
        message: &str,
        options: &[SelectOption],
        default_indices: &[usize],
    ) -> Result<Vec<SelectOption>, InteractiveError> {
        if options.is_empty() {
            return Ok(Vec::new());
        }

        let mut prompt = cliclack::multiselect(message);
        for opt in options.iter() {
            let label = if let Some(desc) = opt.description() {
                format!("{} - {}", opt.label(), desc)
            } else {
                opt.label().to_string()
            };
            let _is_default =
                default_indices.contains(&options.iter().position(|x| x == opt).unwrap());
            prompt = prompt.item(opt.clone(), label, "");
            // Note: cliclack 0.3 multiselect default selections might be handled differently, maybe initial_values.
        }

        prompt
            .interact()
            .map_err(|e| Self::map_cliclack_error(e, "multiselect failed"))
    }
}

impl Logger for CliclackPromptService {
    fn intro(&self, message: &str) -> Result<(), InteractiveError> {
        cliclack::intro(message).map_err(|e| Self::map_cliclack_error(e, "intro failed"))
    }

    fn outro(&self, message: &str) -> Result<(), InteractiveError> {
        cliclack::outro(message).map_err(|e| Self::map_cliclack_error(e, "outro failed"))
    }

    fn cancel_log(&self, message: &str) -> Result<(), InteractiveError> {
        cliclack::outro_cancel(message).map_err(|e| Self::map_cliclack_error(e, "cancel failed"))
    }

    fn log_info(&self, message: &str) -> Result<(), InteractiveError> {
        cliclack::log::info(message).map_err(|e| Self::map_cliclack_error(e, "log failed"))
    }

    fn log_success(&self, message: &str) -> Result<(), InteractiveError> {
        cliclack::log::success(message).map_err(|e| Self::map_cliclack_error(e, "log failed"))
    }

    fn log_warning(&self, message: &str) -> Result<(), InteractiveError> {
        cliclack::log::warning(message).map_err(|e| Self::map_cliclack_error(e, "log failed"))
    }

    fn log_error(&self, message: &str) -> Result<(), InteractiveError> {
        cliclack::log::error(message).map_err(|e| Self::map_cliclack_error(e, "log failed"))
    }

    fn spinner(&self, message: &str) -> Result<Box<dyn Spinner>, InteractiveError> {
        let pb = cliclack::spinner();
        pb.start(message);
        Ok(Box::new(CliclackSpinner {
            inner: Mutex::new(Some(pb)),
        }))
    }
}

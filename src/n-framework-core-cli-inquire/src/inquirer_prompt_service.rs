use std::io::{self, IsTerminal};

use inquire::{Confirm, MultiSelect, Password, Select, Text};
use n_framework_core_cli_abstractions::{InteractiveError, InteractivePrompt, SelectOption};

/// Help message displayed to users during selection prompts.
const SELECT_HELP_MESSAGE: &str = "↑↓ to move, enter to select, type to filter";

#[derive(Debug, Clone, Copy)]
pub struct InquirerPromptService;

impl InquirerPromptService {
    pub fn new() -> Self {
        Self
    }

    fn map_inquire_error(error: inquire::InquireError, context: &str) -> InteractiveError {
        match error {
            inquire::InquireError::OperationCanceled
            | inquire::InquireError::OperationInterrupted => InteractiveError::cancelled(context),
            inquire::InquireError::IO(io_err) => {
                InteractiveError::io(format!("{}: {}", context, io_err))
            }
            _ => InteractiveError::internal(format!("{}: {}", context, error)),
        }
    }

    fn select_index_internal(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<usize, InteractiveError> {
        if options.is_empty() {
            return Err(InteractiveError::validation(
                "no options available for selection",
            ));
        }

        let display_options: Vec<String> = options.iter().map(|opt| opt.to_string()).collect();
        let starting_cursor = default_index.unwrap_or(0).min(options.len() - 1);

        let selected_index = Select::new(message, display_options)
            .with_starting_cursor(starting_cursor)
            .with_help_message(SELECT_HELP_MESSAGE)
            .prompt()
            .map_err(|e| Self::map_inquire_error(e, "selection failed"))?;

        options
            .iter()
            .position(|opt| opt.to_string() == selected_index)
            .ok_or_else(|| InteractiveError::internal("selected option not found in list"))
    }
}

impl Default for InquirerPromptService {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractivePrompt for InquirerPromptService {
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
        let mut prompt = Text::new(message);
        if let Some(default_value) = default {
            prompt = prompt.with_default(default_value);
        }

        prompt
            .prompt()
            .map_err(|e| Self::map_inquire_error(e, "prompt failed"))
    }

    fn confirm(&self, message: &str, default: bool) -> Result<bool, InteractiveError> {
        Confirm::new(message)
            .with_default(default)
            .prompt()
            .map_err(|e| Self::map_inquire_error(e, "confirmation failed"))
    }

    fn password(&self, message: &str) -> Result<String, InteractiveError> {
        Password::new(message)
            .prompt()
            .map_err(|e| Self::map_inquire_error(e, "password prompt failed"))
    }

    fn select(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<SelectOption, InteractiveError> {
        let index = self.select_index_internal(message, options, default_index)?;
        options
            .get(index)
            .cloned()
            .ok_or_else(|| InteractiveError::internal("selected index out of bounds"))
    }

    fn select_index(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<usize, InteractiveError> {
        self.select_index_internal(message, options, default_index)
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

        let display_options: Vec<String> = options.iter().map(|opt| opt.to_string()).collect();

        let selected = MultiSelect::new(message, display_options)
            .with_default(default_indices)
            .with_help_message("↑↓ to move, space to toggle, enter to confirm")
            .prompt()
            .map_err(|e| Self::map_inquire_error(e, "multiselect failed"))?;

        Ok(selected
            .into_iter()
            .filter_map(|label| options.iter().find(|o| o.to_string() == label).cloned())
            .collect())
    }
}

#[cfg(test)]
#[path = "inquirer_prompt_service.tests.rs"]
mod tests;

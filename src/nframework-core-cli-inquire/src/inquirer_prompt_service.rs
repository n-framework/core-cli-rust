use std::io::{self, IsTerminal};

use inquire::{Confirm, Select, Text};
use nframework_core_cli_abstractions::{PromptError, PromptService, SelectOption};

/// Help message displayed to users during selection prompts.
const SELECT_HELP_MESSAGE: &str = "↑↓ to move, enter to select, type to filter";

#[derive(Debug, Clone, Copy)]
pub struct InquirerPromptService;

impl InquirerPromptService {
    pub fn new() -> Self {
        Self
    }

    fn map_inquire_error(error: inquire::InquireError, context: &str) -> PromptError {
        match error {
            inquire::InquireError::OperationCanceled
            | inquire::InquireError::OperationInterrupted => PromptError::cancelled(context),
            inquire::InquireError::IO(io_err) => {
                PromptError::io(format!("{}: {}", context, io_err))
            }
            _ => PromptError::internal(format!("{}: {}", context, error)),
        }
    }

    fn select_index_internal(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<usize, PromptError> {
        if options.is_empty() {
            return Err(PromptError::validation(
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
            .ok_or_else(|| PromptError::internal("selected option not found in list"))
    }
}

impl Default for InquirerPromptService {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptService for InquirerPromptService {
    fn is_interactive(&self) -> bool {
        io::stdin().is_terminal() && io::stdout().is_terminal()
    }

    fn text(&self, message: &str, default: Option<&str>) -> Result<String, PromptError> {
        let mut prompt = Text::new(message);
        if let Some(default_value) = default {
            prompt = prompt.with_default(default_value);
        }

        prompt
            .prompt()
            .map_err(|e| Self::map_inquire_error(e, "prompt failed"))
    }

    fn confirm(&self, message: &str, default: bool) -> Result<bool, PromptError> {
        Confirm::new(message)
            .with_default(default)
            .prompt()
            .map_err(|e| Self::map_inquire_error(e, "confirmation failed"))
    }

    fn select(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<SelectOption, PromptError> {
        let index = self.select_index_internal(message, options, default_index)?;
        options
            .get(index)
            .cloned()
            .ok_or_else(|| PromptError::internal("selected index out of bounds"))
    }

    fn select_index(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<usize, PromptError> {
        self.select_index_internal(message, options, default_index)
    }
}

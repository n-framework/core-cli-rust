use std::io::{self, IsTerminal};

use inquire::{Confirm, Select, Text};
use nframework_core_cli_abstraction::{PromptError, PromptService, SelectOption};

#[derive(Debug, Clone, Copy)]
pub struct InquirerPromptService;

impl InquirerPromptService {
    pub fn new() -> Self {
        Self
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

        prompt.prompt().map_err(|error| match error {
            inquire::InquireError::OperationCanceled
            | inquire::InquireError::OperationInterrupted => {
                PromptError::cancelled("prompt cancelled")
            }
            inquire::InquireError::IO(_) => PromptError::io("io error during prompt"),
            _ => PromptError::internal(format!("prompt failed: {error}")),
        })
    }

    fn confirm(&self, message: &str, default: bool) -> Result<bool, PromptError> {
        Confirm::new(message)
            .with_default(default)
            .prompt()
            .map_err(|error| match error {
                inquire::InquireError::OperationCanceled
                | inquire::InquireError::OperationInterrupted => {
                    PromptError::cancelled("prompt cancelled")
                }
                inquire::InquireError::IO(_) => PromptError::io("io error during prompt"),
                _ => PromptError::internal(format!("prompt failed: {error}")),
            })
    }

    fn select(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: usize,
    ) -> Result<SelectOption, PromptError> {
        if options.is_empty() {
            return Err(PromptError::validation(
                "no options available for selection",
            ));
        }

        let default_index = default_index.min(options.len() - 1);
        let display_options: Vec<String> = options.iter().map(|opt| opt.to_string()).collect();

        let selected = Select::new(message, display_options)
            .with_starting_cursor(default_index)
            .with_help_message("↑↓ to move, enter to select, type to filter")
            .prompt()
            .map_err(|error| match error {
                inquire::InquireError::OperationCanceled
                | inquire::InquireError::OperationInterrupted => {
                    PromptError::cancelled("selection cancelled")
                }
                inquire::InquireError::IO(_) => PromptError::io("io error during selection"),
                _ => PromptError::internal(format!("selection failed: {error}")),
            })?;

        options
            .iter()
            .find(|opt| opt.to_string() == selected)
            .cloned()
            .ok_or_else(|| PromptError::internal("selected option not found in list"))
    }

    fn select_index(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: usize,
    ) -> Result<usize, PromptError> {
        if options.is_empty() {
            return Err(PromptError::validation(
                "no options available for selection",
            ));
        }

        let default_index = default_index.min(options.len() - 1);
        let display_options: Vec<String> = options.iter().map(|opt| opt.to_string()).collect();

        let selected = Select::new(message, display_options)
            .with_starting_cursor(default_index)
            .with_help_message("↑↓ to move, enter to select, type to filter")
            .prompt()
            .map_err(|error| match error {
                inquire::InquireError::OperationCanceled
                | inquire::InquireError::OperationInterrupted => {
                    PromptError::cancelled("selection cancelled")
                }
                inquire::InquireError::IO(_) => PromptError::io("io error during selection"),
                _ => PromptError::internal(format!("selection failed: {error}")),
            })?;

        Ok(options
            .iter()
            .position(|opt| opt.to_string() == selected)
            .unwrap_or(default_index))
    }
}

use std::io::{self, IsTerminal};

use n_framework_core_cli_abstractions::{
    InteractiveError, InteractivePrompt, SelectOption,
};

#[derive(Debug, Clone, Copy)]
pub struct CliclackPromptService;


impl CliclackPromptService {
    pub fn new() -> Self {
        Self
    }

    fn map_cliclack_error(error: std::io::Error, context: &str) -> InteractiveError {
        // Cliclack functions just return std::io::Error.
        // For Interrupted (ctrl+c), we map to Cancelled.
        if error.kind() == std::io::ErrorKind::Interrupted {
            InteractiveError::cancelled(context)
        } else {
            // Preserve the original error message from io::Error
            InteractiveError::io(format!("{}: {}", context, error))
        }
    }

    fn format_option_label(opt: &SelectOption) -> String {
        if let Some(desc) = opt.description() {
            format!("{} - {}", opt.label(), desc)
        } else {
            opt.label().to_string()
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
        if options.is_empty() {
            return Err(InteractiveError::validation(
                "no options available for selection",
            ));
        }

        let mut prompt = cliclack::select(message);

        for opt in options.iter() {
            prompt = prompt.item(opt.clone(), Self::format_option_label(opt), "");
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
            let label = Self::format_option_label(opt);
            prompt = prompt.item(opt.clone(), label, "");
        }

        if !default_indices.is_empty() {
            let initial_values: Vec<SelectOption> = default_indices
                .iter()
                .filter_map(|&i| options.get(i).cloned())
                .collect();
            prompt = prompt.initial_values(initial_values);
        }

        prompt
            .interact()
            .map_err(|e| Self::map_cliclack_error(e, "multiselect failed"))
    }
}


#[cfg(test)]

#[path = "cliclack_prompt_service.tests.rs"]
mod tests;

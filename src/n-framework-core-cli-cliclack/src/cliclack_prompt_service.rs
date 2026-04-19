use std::io::{self, IsTerminal};

use n_framework_core_cli_abstractions::{InteractiveError, InteractivePrompt, SelectOption};

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
}

impl Default for CliclackPromptService {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractivePrompt for CliclackPromptService {
    fn is_interactive(&self) -> bool {
        #[cfg(test)]
        if std::env::var("CORE_CLI_FORCE_INTERACTIVE").is_err() {
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
            prompt = prompt.item(opt.clone(), opt.label(), opt.description().unwrap_or(""));
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

        struct MultiselectHintTheme;

        impl cliclack::Theme for MultiselectHintTheme {
            fn format_footer_with_message(
                &self,
                state: &cliclack::ThemeState,
                message: &str,
            ) -> String {
                let msg = match state {
                    cliclack::ThemeState::Active => {
                        if message.is_empty() {
                            "\x1b[90m↑↓ to move, space to toggle, enter to confirm\x1b[0m"
                                .to_string()
                        } else {
                            format!(
                                "{} \x1b[90m(↑↓ to move, space to toggle, enter to confirm)\x1b[0m",
                                message
                            )
                        }
                    }
                    _ => message.to_string(),
                };

                let color_code = match state {
                    cliclack::ThemeState::Active => "\x1b[36m",
                    cliclack::ThemeState::Cancel => "\x1b[31m",
                    cliclack::ThemeState::Submit => "\x1b[90m",
                    cliclack::ThemeState::Error(_) => "\x1b[33m",
                };
                let reset = "\x1b[0m";

                let formatted = match state {
                    cliclack::ThemeState::Active => format!("{color_code}└{reset}  {msg}"),
                    cliclack::ThemeState::Cancel => {
                        format!("{color_code}└{reset}  Operation cancelled.")
                    }
                    cliclack::ThemeState::Submit => format!("{color_code}│{reset}"),
                    cliclack::ThemeState::Error(err) => format!("{color_code}└{reset}  {err}"),
                };

                format!("{}\n", formatted)
            }
        }

        cliclack::set_theme(MultiselectHintTheme);

        let mut prompt = cliclack::multiselect(message);

        for opt in options.iter() {
            prompt = prompt.item(opt.clone(), opt.label(), opt.description().unwrap_or(""));
        }

        if !default_indices.is_empty() {
            let initial_values: Vec<SelectOption> = default_indices
                .iter()
                .filter_map(|&i| options.get(i).cloned())
                .collect();
            prompt = prompt.initial_values(initial_values);
        }

        let result = prompt
            .interact()
            .map_err(|e| Self::map_cliclack_error(e, "multiselect failed"));

        cliclack::reset_theme();

        result
    }
}

#[cfg(test)]
#[path = "cliclack_prompt_service.tests.rs"]
mod tests;

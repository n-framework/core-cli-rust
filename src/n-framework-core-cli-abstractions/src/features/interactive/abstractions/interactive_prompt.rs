use crate::features::interactive::models::errors::interactive_error::InteractiveError;
use crate::features::interactive::models::select_option::SelectOption;

pub trait InteractivePrompt: Send + Sync {
    /// Returns true if the prompt service is currently running in an interactive terminal.
    fn is_interactive(&self) -> bool;

    /// Prompts the user for a single line of text.
    fn text(&self, message: &str, default: Option<&str>) -> Result<String, InteractiveError>;

    /// Prompts the user for a boolean confirmation.
    fn confirm(&self, message: &str, default: bool) -> Result<bool, InteractiveError>;

    /// Prompts the user for a password/secret without echoing the input.
    fn password(&self, message: &str) -> Result<String, InteractiveError>;

    /// Prompts the user to select an option from a list, returning the selected option.
    fn select(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<SelectOption, InteractiveError>;

    /// Prompts the user to select an option from a list, returning the index of the selected option.
    fn select_index(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<usize, InteractiveError>;

    /// Prompts the user to select multiple options from a list.
    fn multiselect(
        &self,
        message: &str,
        options: &[SelectOption],
        default_indices: &[usize],
    ) -> Result<Vec<SelectOption>, InteractiveError>;
}

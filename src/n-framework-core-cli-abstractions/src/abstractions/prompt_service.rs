use crate::errors::PromptError;
use crate::models::SelectOption;

/// Abstract interface for interactive CLI prompts.
///
/// Implementations must be thread-safe (Send + Sync).
/// Return `PromptError::cancelled()` when users explicitly cancel (e.g., Ctrl+C) or
/// when the operation is interrupted (e.g., via signal).
pub trait PromptService: Send + Sync {
    /// Returns true only when running in a TTY environment where user interaction is possible.
    fn is_interactive(&self) -> bool;

    fn text(&self, message: &str, default: Option<&str>) -> Result<String, PromptError>;

    fn confirm(&self, message: &str, default: bool) -> Result<bool, PromptError>;

    fn select(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<SelectOption, PromptError>;

    fn select_index(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<usize, PromptError>;

    fn multiselect(
        &self,
        message: &str,
        options: &[SelectOption],
        default_indices: &[usize],
    ) -> Result<Vec<SelectOption>, PromptError>;
}

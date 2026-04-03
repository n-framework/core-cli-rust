use crate::errors::PromptError;
use crate::models::SelectOption;

pub trait PromptService: Send + Sync {
    fn is_interactive(&self) -> bool;

    fn text(&self, message: &str, default: Option<&str>) -> Result<String, PromptError>;

    fn confirm(&self, message: &str, default: bool) -> Result<bool, PromptError>;

    fn select(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: usize,
    ) -> Result<SelectOption, PromptError>;

    fn select_index(
        &self,
        message: &str,
        options: &[SelectOption],
        default_index: usize,
    ) -> Result<usize, PromptError>;
}

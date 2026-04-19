use crate::features::interactive::models::errors::interactive_error::InteractiveError;

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct SelectOption {
    label: String,
    value: String,
    description: Option<String>,
}

impl SelectOption {
    /// Creates a new SelectOption and validates that label and value are not empty.
    ///
    /// # Panics
    /// Panics if label or value is empty. Use `try_new` for a non-panicking version.
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self::try_new(label, value).expect("SelectOption label and value cannot be empty")
    }

    /// Safely creates a new SelectOption, returning an error if label or value is empty.
    pub fn try_new(
        label: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, InteractiveError> {
        let label = label.into();
        let value = value.into();

        if label.trim().is_empty() {
            return Err(InteractiveError::validation(
                "SelectOption label cannot be empty",
            ));
        }
        if value.trim().is_empty() {
            return Err(InteractiveError::validation(
                "SelectOption value cannot be empty",
            ));
        }

        Ok(Self {
            label,
            value,
            description: None,
        })
    }

    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl std::fmt::Display for SelectOption {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.description {
            Some(desc) => write!(formatter, "{} - {}", self.label, desc),
            None => write!(formatter, "{}", self.label),
        }
    }
}

#[cfg(test)]
#[path = "select_option.tests.rs"]
mod tests;

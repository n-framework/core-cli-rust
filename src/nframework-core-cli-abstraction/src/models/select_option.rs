#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectOption {
    label: String,
    value: String,
    description: Option<String>,
}

impl SelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        let label = label.into();
        let value = value.into();
        assert!(!label.is_empty(), "label cannot be empty");
        assert!(!value.is_empty(), "value cannot be empty");
        Self {
            label,
            value,
            description: None,
        }
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

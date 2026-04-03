#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliOptionSpec {
    pub id: String,
    pub long: String,
    pub help: Option<String>,
    pub required: bool,
    pub positional_index: Option<usize>,
    pub takes_value: bool,
}

impl CliOptionSpec {
    pub fn new(id: impl Into<String>, long: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            long: long.into(),
            help: None,
            required: false,
            positional_index: None,
            takes_value: true,
        }
    }

    pub fn positional(id: impl Into<String>, index: usize) -> Self {
        Self {
            id: id.into(),
            long: String::new(),
            help: None,
            required: false,
            positional_index: Some(index),
            takes_value: true,
        }
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn flag(mut self) -> Self {
        self.takes_value = false;
        self
    }
}

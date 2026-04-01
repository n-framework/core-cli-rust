#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliOptionSpec {
    pub id: String,
    pub long: String,
    pub help: Option<String>,
    pub required: bool,
}

impl CliOptionSpec {
    pub fn new(id: impl Into<String>, long: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            long: long.into(),
            help: None,
            required: false,
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
}

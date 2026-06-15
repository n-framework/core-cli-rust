use crate::CliOptionSpec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliCommandSpec {
    pub name: String,
    pub about: Option<String>,
    pub long_about: Option<String>,
    pub after_help: Option<String>,
    pub options: Vec<CliOptionSpec>,
    pub subcommands: Vec<CliCommandSpec>,
    pub require_subcommand: bool,
}

impl CliCommandSpec {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            about: None,
            long_about: None,
            after_help: None,
            options: Vec::new(),
            subcommands: Vec::new(),
            require_subcommand: false,
        }
    }

    pub fn with_about(mut self, about: impl Into<String>) -> Self {
        self.about = Some(about.into());
        self
    }

    pub fn with_long_about(mut self, long_about: impl Into<String>) -> Self {
        self.long_about = Some(long_about.into());
        self
    }

    pub fn with_after_help(mut self, after_help: impl Into<String>) -> Self {
        self.after_help = Some(after_help.into());
        self
    }

    pub fn with_option(mut self, option: CliOptionSpec) -> Self {
        self.options.push(option);
        self
    }

    pub fn with_subcommand(mut self, subcommand: CliCommandSpec) -> Self {
        self.subcommands.push(subcommand);
        self
    }

    pub fn require_subcommand(mut self) -> Self {
        self.require_subcommand = true;
        self
    }
}

use crate::nframework_core_cli_abstraction::CliOptionSpec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliCommandSpec {
    pub name: String,
    pub about: Option<String>,
    pub options: Vec<CliOptionSpec>,
    pub subcommands: Vec<CliCommandSpec>,
    pub require_subcommand: bool,
}

impl CliCommandSpec {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            about: None,
            options: Vec::new(),
            subcommands: Vec::new(),
            require_subcommand: false,
        }
    }

    pub fn with_about(mut self, about: impl Into<String>) -> Self {
        self.about = Some(about.into());
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

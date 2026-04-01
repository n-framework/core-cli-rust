use crate::nframework_core_cli_abstraction::CliCommandSpec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliSpec {
    pub name: String,
    pub about: Option<String>,
    pub banner: Option<String>,
    pub commands: Vec<CliCommandSpec>,
    pub require_command: bool,
}

impl CliSpec {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            about: None,
            banner: None,
            commands: Vec::new(),
            require_command: false,
        }
    }

    pub fn with_about(mut self, about: impl Into<String>) -> Self {
        self.about = Some(about.into());
        self
    }

    pub fn with_banner(mut self, banner: impl Into<String>) -> Self {
        self.banner = Some(banner.into());
        self
    }

    pub fn with_command(mut self, command: CliCommandSpec) -> Self {
        self.commands.push(command);
        self
    }

    pub fn require_command(mut self) -> Self {
        self.require_command = true;
        self
    }
}

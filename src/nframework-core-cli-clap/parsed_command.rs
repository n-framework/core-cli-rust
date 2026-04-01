use std::collections::BTreeMap;

use crate::nframework_core_cli_abstraction::Command;

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    name: String,
    args: Vec<String>,
    options: BTreeMap<String, String>,
}

impl ParsedCommand {
    pub fn new(name: String, args: Vec<String>, options: BTreeMap<String, String>) -> Self {
        Self {
            name,
            args,
            options,
        }
    }
}

impl Command for ParsedCommand {
    fn name(&self) -> &str {
        &self.name
    }

    fn args(&self) -> &[String] {
        &self.args
    }

    fn option(&self, name: &str) -> Option<&str> {
        self.options.get(name).map(String::as_str)
    }
}

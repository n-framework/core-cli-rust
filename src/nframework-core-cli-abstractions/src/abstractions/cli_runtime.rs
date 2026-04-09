use std::collections::BTreeMap;

use crate::{CliAdapter, CliRuntimeHandler};

pub struct CliRuntime<C> {
    adapter: Box<dyn CliAdapter>,
    context: C,
    handlers: BTreeMap<String, Box<dyn CliRuntimeHandler<C>>>,
}

impl<C> CliRuntime<C> {
    pub fn new(adapter: Box<dyn CliAdapter>, context: C) -> Self {
        Self {
            adapter,
            context,
            handlers: BTreeMap::new(),
        }
    }

    pub fn register_handler(
        mut self,
        command_name: impl Into<String>,
        handler: impl CliRuntimeHandler<C> + 'static,
    ) -> Self {
        self.handlers.insert(command_name.into(), Box::new(handler));
        self
    }

    pub fn run(&self, input: &[String]) -> Result<(), String> {
        let command = match self.adapter.parse(input) {
            Ok(command) => command,
            Err(error) if error.is_help() => {
                print!("{}", error.message());
                return Ok(());
            }
            Err(error) => return Err(error.to_string()),
        };

        let Some(handler) = self.handlers.get(command.name()) else {
            return Err(format!("unsupported command: {}", command.name()));
        };

        handler.handle(command.as_ref(), &self.context)
    }
}

use n_framework_core_cli_abstractions::{CliAppConfig, CliRuntime, CliRuntimeHandler};

use crate::ClapAdapter;

pub struct ClapCliRuntimeBuilder<C> {
    runtime: CliRuntime<C>,
}

impl<C> ClapCliRuntimeBuilder<C> {
    pub fn new(config: CliAppConfig, context: C) -> Self {
        let adapter = ClapAdapter::from_spec(&config.spec);
        let runtime = CliRuntime::new(Box::new(adapter), context);

        Self { runtime }
    }

    pub fn register_handler(
        mut self,
        command_name: impl Into<String>,
        handler: impl CliRuntimeHandler<C> + 'static,
    ) -> Self {
        self.runtime = self.runtime.register_handler(command_name, handler);
        self
    }

    pub fn build(self) -> CliRuntime<C> {
        self.runtime
    }
}

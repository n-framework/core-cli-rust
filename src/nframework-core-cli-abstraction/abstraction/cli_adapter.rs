use crate::nframework_core_cli_abstraction::{CliAdapterError, Command};

pub trait CliAdapter {
    fn parse(&self, input: &[String]) -> Result<Box<dyn Command>, CliAdapterError>;
}

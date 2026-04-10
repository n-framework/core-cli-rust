use crate::{CliAdapterError, Command};

pub trait CliAdapter {
    fn parse(&self, input: &[String]) -> Result<Box<dyn Command>, CliAdapterError>;
}

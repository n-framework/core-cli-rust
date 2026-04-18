use crate::Command;

pub trait CliRuntimeHandler<C> {
    fn handle(&self, command: &dyn Command, context: &C) -> Result<(), String>;
}

impl<C, F> CliRuntimeHandler<C> for F
where
    F: for<'a> Fn(&'a dyn Command, &C) -> Result<(), String>,
{
    fn handle(&self, command: &dyn Command, context: &C) -> Result<(), String> {
        self(command, context)
    }
}

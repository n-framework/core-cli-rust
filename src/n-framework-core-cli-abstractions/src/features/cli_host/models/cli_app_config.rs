use crate::CliSpec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliAppConfig {
    pub spec: CliSpec,
}

impl CliAppConfig {
    pub fn new(spec: CliSpec) -> Self {
        Self { spec }
    }
}

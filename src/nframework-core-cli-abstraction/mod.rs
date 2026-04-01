pub mod abstraction;
pub mod errors;
pub mod models;

pub use abstraction::{CliAdapter, Command};
pub use errors::CliAdapterError;
pub use models::{CliCommandSpec, CliOptionSpec, CliSpec};

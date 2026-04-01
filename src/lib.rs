#[path = "nframework-core-cli-abstraction/mod.rs"]
pub mod nframework_core_cli_abstraction;

#[path = "nframework-core-cli-clap/mod.rs"]
pub mod nframework_core_cli_clap;

pub use nframework_core_cli_abstraction::{
    CliAdapter, CliAdapterError, CliCommandSpec, CliOptionSpec, CliSpec, Command,
};
pub use nframework_core_cli_clap::ClapAdapter;

use std::collections::BTreeMap;

use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand, error::ErrorKind};

use nframework_core_cli_abstractions::{
    CliAdapter, CliAdapterError, CliCommandSpec, CliOptionSpec, CliSpec, Command,
};

use super::parsed_command::ParsedCommand;

#[derive(Debug, Clone)]
pub struct ClapAdapter {
    definition: ClapCommand,
    root_name: String,
}

impl ClapAdapter {
    pub fn new(definition: ClapCommand) -> Self {
        let root_name = definition.get_name().to_owned();
        Self {
            definition,
            root_name,
        }
    }

    pub fn from_spec(spec: &CliSpec) -> Self {
        Self::new(build_root_command(spec))
    }
}

impl CliAdapter for ClapAdapter {
    fn parse(&self, input: &[String]) -> Result<Box<dyn Command>, CliAdapterError> {
        let mut argv = Vec::with_capacity(input.len() + 1);
        argv.push(self.root_name.clone());
        argv.extend(input.iter().cloned());

        let matches = self
            .definition
            .clone()
            .try_get_matches_from(argv)
            .map_err(map_clap_error)?;

        let (name, terminal_matches) = command_name_and_terminal_matches(&matches, &self.root_name);
        let options = collect_string_options(terminal_matches);

        Ok(Box::new(ParsedCommand::new(name, Vec::new(), options)))
    }
}

fn build_root_command(spec: &CliSpec) -> ClapCommand {
    let mut command = ClapCommand::new(spec.name.clone());
    if let Some(about) = &spec.about {
        command = command.about(about.clone());
    }
    if spec.require_command {
        command = command
            .subcommand_required(true)
            .arg_required_else_help(true);
    }

    for subcommand in &spec.commands {
        command = command.subcommand(build_command(subcommand));
    }

    command
}

fn build_command(spec: &CliCommandSpec) -> ClapCommand {
    let mut command = ClapCommand::new(spec.name.clone());
    if let Some(about) = &spec.about {
        command = command.about(about.clone());
    }
    if spec.require_subcommand {
        command = command
            .subcommand_required(true)
            .arg_required_else_help(true);
    }

    for option in &spec.options {
        command = command.arg(build_option(option));
    }
    for subcommand in &spec.subcommands {
        command = command.subcommand(build_command(subcommand));
    }

    command
}

fn build_option(spec: &CliOptionSpec) -> Arg {
    let mut argument = Arg::new(spec.id.clone());
    if let Some(index) = spec.positional_index {
        argument = argument.index(index);
    } else {
        argument = argument.long(spec.long.clone());
    }
    if let Some(help) = &spec.help {
        argument = argument.help(help.clone());
    }
    if !spec.takes_value {
        argument = argument.action(ArgAction::SetTrue);
    }
    if spec.required {
        argument = argument.required(true);
    }
    argument
}

fn map_clap_error(error: clap::Error) -> CliAdapterError {
    match error.kind() {
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayVersion
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
            CliAdapterError::help(error.to_string())
        }
        _ => CliAdapterError::parse(error.to_string()),
    }
}

fn command_name_and_terminal_matches<'a>(
    matches: &'a ArgMatches,
    root_name: &str,
) -> (String, &'a ArgMatches) {
    let mut names = Vec::<String>::new();
    let mut current_matches = matches;

    while let Some((subcommand_name, subcommand_matches)) = current_matches.subcommand() {
        names.push(subcommand_name.to_owned());
        current_matches = subcommand_matches;
    }

    if names.is_empty() {
        return (root_name.to_owned(), current_matches);
    }

    (names.join("/"), current_matches)
}

fn collect_string_options(matches: &ArgMatches) -> BTreeMap<String, String> {
    let mut options = BTreeMap::<String, String>::new();

    for argument_id in matches.ids() {
        let name = argument_id.as_str();
        if let Ok(Some(value)) = matches.try_get_one::<String>(name) {
            options.insert(name.to_owned(), value.clone());
            continue;
        }
        if let Ok(Some(value)) = matches.try_get_one::<bool>(name)
            && *value
        {
            options.insert(name.to_owned(), "true".to_owned());
        }
    }

    options
}

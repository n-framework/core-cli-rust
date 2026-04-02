use std::{cell::RefCell, rc::Rc};

use nframework_core_cli_abstraction::{
    CliAdapter, CliAppConfig, CliCommandSpec, CliOptionSpec, CliSpec, Command,
};
use nframework_core_cli_clap::{ClapAdapter, ClapCliRuntimeBuilder};

#[test]
fn parses_nested_command_and_option_values() {
    let cli_spec = CliSpec::new("tool").require_command().with_command(
        CliCommandSpec::new("templates")
            .require_subcommand()
            .with_subcommand(CliCommandSpec::new("list"))
            .with_subcommand(
                CliCommandSpec::new("add")
                    .with_option(CliOptionSpec::new("name", "name").required())
                    .with_option(CliOptionSpec::new("url", "url").required()),
            ),
    );
    let adapter = ClapAdapter::from_spec(&cli_spec);

    let command = adapter
        .parse(&[
            "templates".to_owned(),
            "add".to_owned(),
            "--name".to_owned(),
            "official".to_owned(),
            "--url".to_owned(),
            "https://example.com/templates.git".to_owned(),
        ])
        .expect("command should parse");

    assert_eq!(command.name(), "templates/add");
    assert_eq!(command.option("name"), Some("official"));
    assert_eq!(
        command.option("url"),
        Some("https://example.com/templates.git")
    );
}

#[test]
fn returns_help_error_for_help_flag() {
    let cli_spec = CliSpec::new("tool")
        .require_command()
        .with_command(CliCommandSpec::new("templates"));
    let adapter = ClapAdapter::from_spec(&cli_spec);

    let error = match adapter.parse(&["--help".to_owned()]) {
        Ok(_) => panic!("help should be returned as parse error"),
        Err(error) => error,
    };

    assert!(error.is_help());
    assert!(error.message().contains("Usage:"));
}

#[derive(Debug, Default)]
struct RuntimeContext {
    events: Rc<RefCell<Vec<String>>>,
}

#[test]
fn runtime_dispatches_registered_handler() {
    let events = Rc::new(RefCell::<Vec<String>>::new(Vec::new()));
    let cli_spec = CliSpec::new("tool").require_command().with_command(
        CliCommandSpec::new("templates").with_subcommand(
            CliCommandSpec::new("add").with_option(CliOptionSpec::new("name", "name").required()),
        ),
    );
    let runtime = ClapCliRuntimeBuilder::new(
        CliAppConfig::new(cli_spec),
        RuntimeContext {
            events: events.clone(),
        },
    )
    .register_handler(
        "templates/add",
        |command: &dyn Command, context: &RuntimeContext| {
            let name = command
                .option("name")
                .ok_or_else(|| "missing --name".to_owned())?;
            context.events.borrow_mut().push(name.to_owned());
            Ok(())
        },
    )
    .build();

    runtime
        .run(&[
            "templates".to_owned(),
            "add".to_owned(),
            "--name".to_owned(),
            "official".to_owned(),
        ])
        .expect("runtime should dispatch command");

    assert_eq!(*events.borrow(), vec!["official".to_owned()]);
}

#[test]
fn runtime_returns_error_for_unregistered_command() {
    let cli_spec = CliSpec::new("tool").require_command().with_command(
        CliCommandSpec::new("templates").with_subcommand(CliCommandSpec::new("list")),
    );
    let runtime =
        ClapCliRuntimeBuilder::new(CliAppConfig::new(cli_spec), RuntimeContext::default()).build();

    let error = runtime
        .run(&["templates".to_owned(), "list".to_owned()])
        .expect_err("missing handler should return an error");

    assert!(error.contains("unsupported command"));
}

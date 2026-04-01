use nframework_core_cli_abstraction::{CliAdapter, CliCommandSpec, CliOptionSpec, CliSpec};
use nframework_core_cli_clap::ClapAdapter;

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

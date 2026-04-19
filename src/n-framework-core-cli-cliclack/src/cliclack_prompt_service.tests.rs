use super::CliclackPromptService;
use n_framework_core_cli_abstractions::{InteractivePrompt, SelectOption};

#[test]
fn test_cliclack_select_empty_options_returns_error() {
    let service = CliclackPromptService::new();
    let result = service.select("Pick one", &[], None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().message(),
        "no options available for selection"
    );
}

#[test]
fn test_cliclack_format_option_label() {
    let opt = SelectOption::new("Label", "value");
    assert_eq!(CliclackPromptService::format_option_label(&opt), "Label");

    let opt_with_desc = SelectOption::new("Label", "value").with_description("Description");
    assert_eq!(
        CliclackPromptService::format_option_label(&opt_with_desc),
        "Label - Description"
    );
}

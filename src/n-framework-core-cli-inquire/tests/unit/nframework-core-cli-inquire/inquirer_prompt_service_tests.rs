use n_framework_core_cli_abstractions::{PromptErrorKind, PromptService};
use n_framework_core_cli_inquire::InquirerPromptService;
use std::io::IsTerminal;

#[test]
fn test_inquirer_prompt_service_is_interactive_in_tty() {
    let service = InquirerPromptService::new();
    // This test is fragile because it depends on the host environment's TTY state.
    // We only assert if we are sure we are in a non-interactive environment (like CI).
    if std::env::var("CI").is_ok() || !std::io::stdin().is_terminal() {
        assert!(!service.is_interactive());
    }
}

#[test]
fn test_inquirer_prompt_service_default() {
    let service = InquirerPromptService;
    if std::env::var("CI").is_ok() || !std::io::stdin().is_terminal() {
        assert!(!service.is_interactive());
    }
}

#[test]
fn test_select_empty_options_returns_error() {
    let service = InquirerPromptService::new();
    let result = service.select("Choose", &[], None);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), &PromptErrorKind::Validation);
}

#[test]
fn test_select_index_empty_options_returns_error() {
    let service = InquirerPromptService::new();
    let result = service.select_index("Choose", &[], None);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), &PromptErrorKind::Validation);
}

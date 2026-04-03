use nframework_core_cli_inquire::InquirerPromptService;
use nframework_core_cli_abstraction::{PromptError, PromptService};

#[test]
fn test_inquirer_prompt_service_is_interactive_in_tty() {
    let service = InquirerPromptService::new();
    // In a test environment, stdin/stdout are not TTYs
    assert!(!service.is_interactive());
}

#[test]
fn test_inquirer_prompt_service_default() {
    let service = InquirerPromptService::default();
    assert!(!service.is_interactive());
}

#[test]
fn test_select_empty_options_returns_error() {
    let service = InquirerPromptService::new();
    let result = service.select("Choose", &[], None);
    assert!(result.is_err());
    match result {
        Err(PromptError { .. }) => {},
        _ => panic!("Expected validation error"),
    }
}

#[test]
fn test_select_index_empty_options_returns_error() {
    let service = InquirerPromptService::new();
    let result = service.select_index("Choose", &[], None);
    assert!(result.is_err());
}

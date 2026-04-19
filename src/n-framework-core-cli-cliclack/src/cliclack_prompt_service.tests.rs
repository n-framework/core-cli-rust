use super::CliclackPromptService;
use n_framework_core_cli_abstractions::{InteractiveErrorKind, InteractivePrompt, Spinner};
use std::io;

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
fn test_cliclack_select_index_empty_options_returns_error() {
    let service = CliclackPromptService::new();
    let result = service.select_index("Pick one", &[], None);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().message(),
        "no options available for selection"
    );
}

#[test]
fn test_cliclack_map_error() {
    let err = io::Error::new(io::ErrorKind::Interrupted, "ctrl+c");
    let mapped = CliclackPromptService::map_cliclack_error(err, "context");
    assert_eq!(*mapped.kind(), InteractiveErrorKind::Cancelled);
    assert_eq!(mapped.message(), "context");

    let err = io::Error::new(io::ErrorKind::Other, "boom");
    let mapped = CliclackPromptService::map_cliclack_error(err, "context");
    assert_eq!(*mapped.kind(), InteractiveErrorKind::Io);
    assert!(mapped.message().contains("context: boom"));
}

#[test]
fn test_cliclack_spinner_poison_recovery() {
    use crate::cliclack_spinner::CliclackSpinner;
    use std::sync::Mutex;

    let spinner = CliclackSpinner {
        inner: Mutex::new(None),
    };

    // Poison the mutex
    let _ = std::panic::catch_unwind(|| {
        let _guard = spinner.inner.lock().unwrap();
        panic!("poisoning");
    });

    assert!(spinner.inner.is_poisoned());

    // Verify recovery via into_inner() in methods
    spinner.success("done"); // Should not panic
    spinner.error("failed"); // Should not panic
    spinner.cancel("aborted"); // Should not panic
    spinner.stop("stopped"); // Should not panic
}

use nframework_core_cli_abstraction::errors::PromptError;

#[test]
fn test_prompt_error_constructors() {
    let cancelled = PromptError::cancelled("test");
    assert!(cancelled.is_cancelled());
    assert_eq!(cancelled.message(), "test");

    let io = PromptError::io("io error");
    assert!(!io.is_cancelled());
    assert_eq!(io.message(), "io error");

    let validation = PromptError::validation("validation error");
    assert!(!validation.is_cancelled());
    assert_eq!(validation.message(), "validation error");

    let internal = PromptError::internal("internal error");
    assert!(!internal.is_cancelled());
    assert_eq!(internal.message(), "internal error");
}

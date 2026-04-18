use super::InteractiveError;

#[test]
fn test_prompt_error_constructors() {
    let cancelled = InteractiveError::cancelled("test");
    assert!(cancelled.is_cancelled());
    assert_eq!(cancelled.message(), "test");

    let io = InteractiveError::io("io error");
    assert!(!io.is_cancelled());
    assert_eq!(io.message(), "io error");

    let validation = InteractiveError::validation("validation error");
    assert!(!validation.is_cancelled());
    assert_eq!(validation.message(), "validation error");

    let internal = InteractiveError::internal("internal error");
    assert!(!internal.is_cancelled());
    assert_eq!(internal.message(), "internal error");
}

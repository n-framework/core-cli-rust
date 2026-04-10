use n_framework_core_cli_abstractions::models::SelectOption;

#[test]
fn test_select_option_new() {
    let option = SelectOption::new("Label", "value");
    assert_eq!(option.label(), "Label");
    assert_eq!(option.value(), "value");
    assert_eq!(option.description(), None);
}

#[test]
fn test_select_option_with_description() {
    let option = SelectOption::new("Label", "value").with_description("Description");
    assert_eq!(option.label(), "Label");
    assert_eq!(option.value(), "value");
    assert_eq!(option.description(), Some("Description"));
}

#[test]
#[should_panic(expected = "label cannot be empty")]
fn test_select_option_empty_label_panics() {
    SelectOption::new("", "value");
}

#[test]
#[should_panic(expected = "value cannot be empty")]
fn test_select_option_empty_value_panics() {
    SelectOption::new("Label", "");
}

#[test]
fn test_select_option_display() {
    let option = SelectOption::new("Label", "value");
    assert_eq!(option.to_string(), "Label");

    let option_with_desc = SelectOption::new("Label", "value").with_description("Description");
    assert_eq!(option_with_desc.to_string(), "Label - Description");
}

use patternfly_yew::prelude::*;

pub fn make_email_validator() -> Validator<String, ValidationResult> {
    Validator::from(|ctx: ValidationContext<String>| {
        if ctx.initial {
            ValidationResult::default()
        } else if ctx.value.is_empty() {
            ValidationResult::error("Must not be empty")
        } else {
            ValidationResult::new(InputState::Success, "Correct")
        }
    })
}

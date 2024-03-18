use patternfly_yew::prelude::*;
use regex::Regex;

pub fn make_email_validator() -> Validator<String, ValidationResult> {
    let email_regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();

    Validator::from(move |ctx: ValidationContext<String>| {
        if ctx.initial {
            ValidationResult::default()
        } else if ctx.value.is_empty() {
            ValidationResult::error("Must not be empty")
        } else if !email_regex.is_match(&ctx.value) {
            ValidationResult::error("Not a valid email format")
        } else {
            ValidationResult::new(InputState::Success, "Correct")
        }
    })
}

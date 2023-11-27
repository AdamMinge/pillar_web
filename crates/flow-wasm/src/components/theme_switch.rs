use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ThemeSwitch)]
pub fn theme_switch() -> Html {
    let onthemeswitch = use_callback((), |state: bool, ()| match state {
        true => gloo_utils::document_element().set_class_name("pf-v5-theme-dark"),
        false => gloo_utils::document_element().set_class_name(""),
    });

    html! {
        <Switch onchange={onthemeswitch} label="Dark Theme" />
    }
}

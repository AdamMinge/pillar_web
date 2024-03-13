use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::components;

#[function_component(AboutDropdown)]
pub fn about_dropdown() -> Html {
    let backdropper = use_backdrop();
    let onabout = use_callback((), move |_, ()| {
        if let Some(backdropper) = &backdropper {
            backdropper.open(html!(<components::AboutBullseye/>));
        }
    });

    html! {
        <Dropdown
            position={Position::Right}
            icon={Icon::QuestionCircle}
            variant={MenuToggleVariant::Plain}
        >
            <MenuAction onclick={onabout}>{"About"}</MenuAction>
        </Dropdown>
    }
}

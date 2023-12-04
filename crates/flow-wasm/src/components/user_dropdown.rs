use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::app::{AppRoute, UserRoute};
use crate::hooks::use_router;
use crate::hooks::use_user_context;

#[function_component(UserDropdown)]
pub fn user_dropdown() -> Html {
    let user_ctx = use_user_context();

    let login_callback = use_router(AppRoute::User(UserRoute::Login));
    let register_callback = use_router(AppRoute::User(UserRoute::Signup));

    let dropdown_content = if user_ctx.is_authenticated() {
        vec![]
    } else {
        vec![
            html_nested! { <MenuAction onclick={login_callback}>{"Login"}</MenuAction> },
            html_nested! { <MenuAction onclick={register_callback}>{"Register"}</MenuAction> },
        ]
    };

    html! {
        <Dropdown
            position={Position::Right}
            icon={Icon::User}
            variant={MenuToggleVariant::Plain}
        >
            { for dropdown_content.into_iter() }
        </Dropdown>
    }
}

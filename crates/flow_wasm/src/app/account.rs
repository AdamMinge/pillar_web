use crate::hooks::use_router;
use crate::route::{AppRoute, UserRoute};

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(AppAccount)]
pub fn app_account() -> Html {
    let login_callback = use_router(AppRoute::User(UserRoute::Login));
    let register_callback = use_router(AppRoute::User(UserRoute::Signup));

    let dropdown_content = if
    /*is_authenticated()*/
    false {
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

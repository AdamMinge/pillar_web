use crate::hooks::use_router;
use crate::route::{AppRoute, UserRoute};

use flow_api::components::{Authenticated, NotAuthenticated};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(AppAccount)]
pub fn app_account() -> Html {
    let login_callback = use_router(AppRoute::User(UserRoute::Login));
    let register_callback = use_router(AppRoute::User(UserRoute::Signup));

    html! {
        <>
            <Authenticated>
            <Dropdown
                position={Position::Right}
                icon={Icon::User}
                variant={MenuToggleVariant::Plain}
            >

            </Dropdown>
            </Authenticated>

            <NotAuthenticated>
            <Dropdown
                position={Position::Right}
                icon={Icon::User}
                variant={MenuToggleVariant::Plain}
            >
                <MenuAction onclick={login_callback}>{"Login"}</MenuAction>
                <MenuAction onclick={register_callback}>{"Register"}</MenuAction>
            </Dropdown>
            </NotAuthenticated>
        </>
    }
}

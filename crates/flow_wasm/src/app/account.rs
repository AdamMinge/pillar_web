use crate::routes::AuthRoute;

use flow_api::components::{Authenticated, NotAuthenticated};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AppAccount)]
pub fn app_account() -> Html {
    let navigator = use_navigator().unwrap();

    let login_callback = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&AuthRoute::Login))
    };

    let register_callback = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&AuthRoute::Signup))
    };

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

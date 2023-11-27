pub mod index;
pub mod user;

use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};
use yew_nested_router::Target;

use crate::components;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum UserRoute {
    #[default]
    Login,
    Register,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[default]
    Index,
    User(UserRoute),
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BackdropViewer>
            <ToastViewer>
                <Router<AppRoute> default={AppRoute::Index}>
                    <components::UserContextProvider>
                        <RouterSwitch<AppRoute> render={switch_app_route} />
                    </components::UserContextProvider>
                </Router<AppRoute>>
            </ToastViewer>
        </BackdropViewer>
    }
}

fn switch_app_route(target: AppRoute) -> Html {
    let user = |target: UserRoute| match target {
        UserRoute::Login => html! {<user::Login/>},
        UserRoute::Register => html! {<user::Register/>},
    };

    match target {
        AppRoute::Index => html! {<components::AppPage><index::Index/></components::AppPage>},

        AppRoute::User(_) => {
            html!(
                <components::AppPage>
                    <Scope<AppRoute, UserRoute> mapper={AppRoute::mapper_user}>
                        <RouterSwitch<UserRoute> render={user}/>
                    </Scope<AppRoute, UserRoute>>
                </components::AppPage>
            )
        }
    }
}

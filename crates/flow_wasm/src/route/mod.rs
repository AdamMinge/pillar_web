mod user;

pub use user::{switch_user_route, UserRoute};

use crate::app::{index::IndexPage, AppPage};

use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};
use yew_nested_router::Target;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[default]
    Index,
    User(user::UserRoute),
}

pub fn switch_app_route(target: AppRoute) -> Html {
    match target {
        AppRoute::Index => html! {
            <AppPage>
                <IndexPage/>
            </AppPage>
        },

        AppRoute::User(_) => {
            html!(
                <Scope<AppRoute, user::UserRoute> mapper={AppRoute::mapper_user}>
                    <RouterSwitch<user::UserRoute> render={user::switch_user_route}/>
                </Scope<AppRoute, user::UserRoute>>
            )
        }
    }
}

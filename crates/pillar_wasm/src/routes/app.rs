use super::auth::{switch_auth_route, AuthRoute};
use crate::app::{index::IndexPage, AppPage};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/auth/*")]
    Auth,
}

pub fn switch_app_route(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! {
            <AppPage>
                <IndexPage/>
            </AppPage>
        },

        AppRoute::Auth => {
            html!(
                <pillar_api::router::NoAuthenticatedSwitch<AppRoute, AuthRoute> render={switch_auth_route} redirect={AppRoute::Home}/>
            )
        }
    }
}

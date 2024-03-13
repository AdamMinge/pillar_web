pub mod index;
pub mod user;

use crate::components;
use patternfly_yew::prelude::*;

use dotenv_codegen::dotenv;
use url::Url;
use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};
use yew_nested_router::Target;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum UserRoute {
    #[default]
    Login,
    Signup,
    #[target(rename = "recovery")]
    PasswordRecovery,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
    #[default]
    Index,
    User(UserRoute),
}

#[function_component(App)]
pub fn app() -> Html {
    let config = flow_api::api::Config {
        api_root: Url::parse(dotenv!("API_ROOT")).unwrap(),
        api_token: Some(dotenv!("API_KEY").to_string()),
    };

    html! {
        <flow_api::components::Api config={config}>
            <BackdropViewer>
                <ToastViewer>
                    <Router<AppRoute> default={AppRoute::Index}>
                        <RouterSwitch<AppRoute> render={switch_app_route} />
                    </Router<AppRoute>>
                </ToastViewer>
            </BackdropViewer>
        </flow_api::components::Api>
    }
}

fn switch_app_route(target: AppRoute) -> Html {
    let user = |target: UserRoute| match target {
        UserRoute::Login => {
            html! {
            <user::LoginPage<UserRoute>
                signup={UserRoute::Signup}
                recovery={UserRoute::PasswordRecovery}
            />}
        }
        UserRoute::Signup => html! {
            <user::SignupPage<UserRoute>
                login={UserRoute::Login}
            />
        },
        UserRoute::PasswordRecovery => html! {<user::PasswordRecoveryPage/>},
    };

    match target {
        AppRoute::Index => html! {
            <components::AppPage>
                <index::IndexPage/>
            </components::AppPage>
        },

        AppRoute::User(_) => {
            html!(
                <Scope<AppRoute, UserRoute> mapper={AppRoute::mapper_user}>
                    <RouterSwitch<UserRoute> render={user}/>
                </Scope<AppRoute, UserRoute>>
            )
        }
    }
}

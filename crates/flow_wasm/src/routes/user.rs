use crate::app::{user::LoginPage, user::PasswordRecoveryPage, user::SignupPage};

use yew::prelude::*;
use yew_nested_router::Target;

#[derive(Debug, Default, Clone, PartialEq, Eq, Target)]
pub enum UserRoute {
    #[default]
    Login,
    Signup,
    #[target(rename = "recovery")]
    PasswordRecovery,
}

pub fn switch_user_route(target: UserRoute) -> Html {
    match target {
        UserRoute::Login => html! {
            <LoginPage/>
        },
        UserRoute::Signup => html! {
            <SignupPage/>

        },
        UserRoute::PasswordRecovery => html! {<PasswordRecoveryPage/>},
    }
}

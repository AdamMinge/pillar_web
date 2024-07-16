use crate::app::{
    auth::ForgotPasswordPage, auth::LoginPage, auth::PasswordRecoveryPage, auth::SignupPage,
};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AuthRoute {
    #[at("/auth/login")]
    Login,
    #[at("/auth/signup")]
    Signup,
    #[at("/auth/recovery")]
    ForgotRecovery,
    #[at("/auth/recovery/:token")]
    PasswordRecovery { token: String },
}

pub fn switch_auth_route(routes: AuthRoute) -> Html {
    match routes {
        AuthRoute::Login => html! {
            <LoginPage/>
        },
        AuthRoute::Signup => html! {
            <SignupPage/>

        },
        AuthRoute::ForgotRecovery => html! {<ForgotPasswordPage/>},
        AuthRoute::PasswordRecovery { .. } => html! {<PasswordRecoveryPage/>},
    }
}

use super::missing_context;
use crate::context::AuthContext;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(NotAuthenticated)]
pub fn not_authenticated(props: &Props) -> Html {
    let auth = use_context::<AuthContext>();

    match auth {
        None => missing_context(),
        Some(AuthContext::NotInitialized) => html!(),
        Some(AuthContext::NotAuthenticated { .. } | AuthContext::Failed(..)) => {
            html!({ for props.children.iter() })
        }
        Some(AuthContext::Authenticated { .. }) => {
            html!()
        }
    }
}

use super::error::missing_context;
use crate::context::AuthContext;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AuthenticatedProperties {
    pub children: Children,
}

#[function_component(Authenticated)]
pub fn authenticated(props: &AuthenticatedProperties) -> Html {
    let auth = use_context::<AuthContext>();

    html!(
        if let Some(auth) = auth {
            if let AuthContext::Authenticated{..} = auth {
                { for props.children.iter() }
            }
        } else {
            { missing_context() }
        }
    )
}

use crate::context::AuthContext;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct AuthenticatedSwitchProps<Re, Ro>
where
    Re: Routable,
    Ro: Routable,
{
    pub render: Callback<Ro, Html>,
    pub redirect: Re,
    #[prop_or_default]
    pub pathname: Option<String>,
}

#[function_component]
pub fn AuthenticatedSwitch<Re, Ro>(props: &AuthenticatedSwitchProps<Re, Ro>) -> Html
where
    Re: Routable + 'static,
    Ro: Routable + 'static,
{
    let route = use_route::<Ro>();
    let auth = use_context::<AuthContext>();

    let route = props
        .pathname
        .as_ref()
        .and_then(|p| Ro::recognize(p))
        .or(route);

    match route {
        Some(route) => {
            if let Some(auth) = auth {
                if let AuthContext::Authenticated { .. } = auth {
                    props.render.emit(route)
                } else {
                    html!(<Redirect<Re> to={props.redirect.clone()}/>)
                }
            } else {
                Html::default()
            }
        }
        None => Html::default(),
    }
}

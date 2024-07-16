use crate::context::AuthContext;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct NoAuthenticatedSwitchProps<Re, Ro>
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
pub fn NoAuthenticatedSwitch<Re, Ro>(props: &NoAuthenticatedSwitchProps<Re, Ro>) -> Html
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
                    html!(<Redirect<Re> to={props.redirect.clone()}/>)
                } else {
                    props.render.emit(route)
                }
            } else {
                Html::default()
            }
        }
        None => Html::default(),
    }
}

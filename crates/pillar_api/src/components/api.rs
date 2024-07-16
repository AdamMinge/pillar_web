use crate::api::{Client, Config};
use crate::context::{AuthContext, ClientContext, Reason};

use yew::prelude::*;

pub enum ApiMsg {
    Context(AuthContext),
}

#[derive(Clone, Debug, Properties)]
pub struct ApiProperties {
    pub config: Config,

    #[prop_or_default]
    pub children: Children,
}

impl PartialEq for ApiProperties {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
    }
}

pub struct Api {
    context: AuthContext,
    client: ClientContext,
    config: Config,
}

impl Component for Api {
    type Message = ApiMsg;
    type Properties = ApiProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(ApiMsg::Context);

        let config = Self::make_config(ctx.props());
        let mut client = Client::new(move |s| callback.emit(s));
        client
            .configure(config.clone())
            .expect("Client configuration failed");

        Self {
            context: AuthContext::NotAuthenticated {
                reason: Reason::NewSession,
            },
            client: ClientContext::new(client),
            config,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Context(context) => {
                if self.context != context {
                    self.context = context;
                    return true;
                }
            }
        }
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _: &Self::Properties) -> bool {
        let config = Self::make_config(ctx.props());
        if self.config != config {
            self.client
                .configure(config.clone())
                .expect("Client configuration failed");
            self.config = config;
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <>
                <ContextProvider<AuthContext> context={self.context.clone()} >
                    <ContextProvider<ClientContext> context={self.client.clone()}>
                        { for ctx.props().children.iter() }
                    </ContextProvider<ClientContext>>
                </ContextProvider<AuthContext>>
            </>
        )
    }
}

impl Api {
    fn make_config(props: &ApiProperties) -> Config {
        Config {
            ..props.config.clone()
        }
    }
}

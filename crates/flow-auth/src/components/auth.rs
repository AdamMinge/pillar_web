use crate::agent::{AgentConfiguration, LoginConfig, LogoutConfig};
use crate::context::{AgentContext, AuthContext};
use std::time::Duration;
use yew::prelude::*;

pub enum Msg {
    Context(AuthContext),
}

#[derive(Clone, Debug, Properties)]
pub struct AuthProperties {
    #[prop_or(Duration::from_secs(30))]
    pub grace_period: Duration,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub login_options: Option<LoginConfig>,
    #[prop_or_default]
    pub logout_options: Option<LogoutConfig>,
}

impl PartialEq for AuthProperties {
    fn eq(&self, other: &Self) -> bool {
        self.grace_period == other.grace_period && self.children == other.children
    }
}

pub struct Auth {
    context: AuthContext,
    agent: AgentContext,
    config: AgentConfiguration,
}

impl Component for Auth {
    type Message = Msg;
    type Properties = AuthProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let config = Self::make_config(ctx.props());
        let callback = ctx.link().callback(Msg::Context);

        let agent = crate::agent::Agent::new(move |s| callback.emit(s));
        agent.configure(config.clone());

        Self {
            context: AuthContext::NotInitialized,
            agent: AgentContext::new(agent),
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
            let _ = self.agent.configure(config.clone());
            self.config = config;
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <>
                <ContextProvider<AuthContext> context={self.context.clone()} >
                    <ContextProvider<AgentContext> context={self.agent.clone()}>
                        { for ctx.props().children.iter() }
                    </ContextProvider<AgentContext>>
                </ContextProvider<AuthContext>>
            </>
        )
    }
}

impl Auth {
    fn make_config(props: &AuthProperties) -> AgentConfiguration {
        AgentConfiguration {
            grace_period: props.grace_period,
        }
    }
}

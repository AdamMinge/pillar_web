pub mod client;
pub mod config;

pub use client::*;
pub use config::*;

use crate::context::{AuthContext, Authentication};
use gloo_timers::callback::Timeout;
use js_sys::Date;
use num_traits::cast::ToPrimitive;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

#[derive(Debug, Clone, Default)]
pub struct LoginConfig {}

#[derive(Debug, Clone, Default)]
pub struct LogoutConfig {}

#[doc(hidden)]
pub enum Msg {
    Configure(AgentConfiguration),
    Login(Option<LoginConfig>),
    Logout(Option<LogoutConfig>),
    Refresh,
}

#[derive(Clone, Debug)]
pub struct Agent {
    tx: Sender<Msg>,
}

impl Agent {
    pub fn new<F>(state_callback: F) -> Self
    where
        F: Fn(AuthContext) + 'static,
    {
        let (tx, rx) = channel(128);

        let inner = InnerAgent::new(tx.clone(), state_callback);
        inner.spawn(rx);

        Self { tx }
    }

    pub fn configure(&self, config: AgentConfiguration) -> bool {
        self.tx.try_send(Msg::Configure(config)).is_err()
    }

    pub fn login(&self, config: Option<LoginConfig>) -> bool {
        self.tx.try_send(Msg::Login(config)).is_err()
    }

    pub fn logout(&self, config: Option<LogoutConfig>) -> bool {
        self.tx.try_send(Msg::Logout(config)).is_err()
    }
}

#[derive(Clone, Debug)]
pub struct InnerConfiguration {
    pub grace_period: Duration,
}

impl InnerConfiguration {
    pub fn new(config: AgentConfiguration) -> Self {
        Self {
            grace_period: config.grace_period,
        }
    }
}

struct InnerAgent {
    tx: Sender<Msg>,
    state_callback: Callback<AuthContext>,
    config: Option<InnerConfiguration>,
    client: Option<AuthClient>,
    state: AuthContext,
    timeout: Option<Timeout>,
}

impl InnerAgent {
    pub fn new<F>(tx: Sender<Msg>, state_callback: F) -> Self
    where
        F: Fn(AuthContext) + 'static,
    {
        Self {
            tx,
            state_callback: Callback::from(state_callback),
            config: None,
            client: None,
            state: AuthContext::NotInitialized,
            timeout: None,
        }
    }

    fn spawn(self, rx: Receiver<Msg>) {
        spawn_local(async move {
            self.run(rx).await;
        })
    }

    async fn run(mut self, mut rx: Receiver<Msg>) {
        loop {
            match rx.recv().await {
                Some(msg) => self.process(msg).await,
                None => {
                    break;
                }
            }
        }
    }

    async fn process(&mut self, msg: Msg) {
        match msg {
            Msg::Configure(config) => self.configure(config).await,
            Msg::Login(config) => self.login(config).await,
            Msg::Logout(config) => self.logout(config).await,
            Msg::Refresh => self.refresh().await,
        }
    }

    fn update_state(&mut self, state: AuthContext) {
        if let AuthContext::Authenticated(Authentication {
            expires: Some(expires),
            ..
        }) = &state
        {
            let grace = self
                .config
                .as_ref()
                .map(|c| c.grace_period)
                .unwrap_or_default();
            let now = Date::now() / 1000f64;
            let diff = *expires as f64 - now - grace.as_secs_f64();

            let tx = self.tx.clone();
            if diff > 0f64 {
                let millis = (diff * 1000f64).to_i32().unwrap_or(i32::MAX);
                self.timeout = Some(Timeout::new(millis as u32, move || {
                    let _ = tx.try_send(Msg::Refresh);
                }));
            } else {
                let _ = tx.try_send(Msg::Refresh);
            }
        } else {
            self.timeout = None;
        }

        self.notify_state(state.clone());

        self.state = state;
    }

    fn notify_state(&self, state: AuthContext) {
        self.state_callback.emit(state);
    }

    async fn configure(&mut self, config: AgentConfiguration) {}

    async fn refresh(&mut self) {
        log::debug!("Refresh");
    }

    async fn login(&mut self, config: Option<LoginConfig>) {
        log::debug!("Logout: {:?}", config);
    }

    async fn logout(&mut self, config: Option<LogoutConfig>) {
        log::debug!("Logout: {:?}", config);
    }
}

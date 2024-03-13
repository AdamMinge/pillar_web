use super::{state, Config, Error};
use crate::context::{AuthContext, Authentication, Reason};
use crate::types::{ErrorResponse, LoginRequest, LoginResponse, RefreshRequest, RefreshResponse};
use reqwest::{header::HeaderName, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use yew::Callback;

const LOGIN_ROUTE: &str = "auth/login/";
const REFRESH_ROUTE: &str = "auth/login/refresh";

#[derive(Clone, Debug)]
pub struct Client {
    inner: InnerClient,
}

impl Client {
    pub fn new<F>(auth_callback: F) -> Self
    where
        F: Fn(AuthContext) + 'static,
    {
        Self {
            inner: InnerClient::new(auth_callback),
        }
    }

    pub fn configure(&mut self, config: Config) -> Result<(), Error> {
        self.inner.configure(config)
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), Error> {
        self.inner.login(username, password).await
    }

    pub async fn logout(&mut self) -> Result<(), Error> {
        self.inner.logout().await
    }

    pub async fn refresh(&mut self) -> Result<(), Error> {
        self.inner.refresh().await
    }

    async fn delete<T>(&mut self, route: &str) -> Result<T, Error>
    where
        T: DeserializeOwned + 'static + std::fmt::Debug,
    {
        self.request(reqwest::Method::DELETE, route, ()).await
    }

    async fn get<T>(&mut self, route: &str) -> Result<T, Error>
    where
        T: DeserializeOwned + 'static + std::fmt::Debug,
    {
        self.request(reqwest::Method::GET, route, ()).await
    }

    async fn post<B, T>(&mut self, route: &str, body: B) -> Result<T, Error>
    where
        T: DeserializeOwned + 'static + std::fmt::Debug,
        B: Serialize + Clone + std::fmt::Debug,
    {
        self.request(reqwest::Method::POST, route, body).await
    }

    async fn put<B, T>(&mut self, route: &str, body: B) -> Result<T, Error>
    where
        T: DeserializeOwned + 'static + std::fmt::Debug,
        B: Serialize + Clone + std::fmt::Debug,
    {
        self.request(reqwest::Method::PUT, route, body).await
    }

    async fn request<B, T>(
        &mut self,
        method: reqwest::Method,
        route: &str,
        body: B,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned + 'static + std::fmt::Debug,
        B: Serialize + Clone + std::fmt::Debug,
    {
        let result = self
            .inner
            .request::<B, T>(method.clone(), route, body.clone())
            .await;
        match result {
            Err(Error::Unauthorized) => {
                if let Ok(()) = self.refresh().await {
                    self.inner.request::<B, T>(method, route, body).await
                } else {
                    Err(Error::Unauthorized)
                }
            }
            any => any,
        }
    }
}

#[derive(Clone, Debug)]
struct InnerClient {
    config: Option<Config>,
    callback: Callback<AuthContext>,
    context: AuthContext,
}

impl InnerClient {
    fn new<F>(auth_callback: F) -> Self
    where
        F: Fn(AuthContext) + 'static,
    {
        Self {
            config: None,
            callback: Callback::from(auth_callback),
            context: AuthContext::NotInitialized,
        }
    }

    fn configure(&mut self, config: Config) -> Result<(), Error> {
        self.config = Some(config);

        if matches!(self.context, AuthContext::NotInitialized) {
            self.restore_state()?
        }

        Ok(())
    }

    fn restore_state(&mut self) -> Result<(), Error> {
        let context = state::get_from_store::<AuthContext, _>(state::STORAGE_KEY_AUTH)?;

        if let Some(AuthContext::Authenticated(auth)) = context {
            self.update_context(AuthContext::Authenticated(auth))?;
        } else {
            self.update_context(AuthContext::NotAuthenticated {
                reason: Reason::NewSession,
            })?;
        }

        Ok(())
    }

    async fn login(&mut self, email: &str, password: &str) -> Result<(), Error> {
        let body = LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let response = self
            .request::<LoginRequest, LoginResponse>(reqwest::Method::POST, LOGIN_ROUTE, body)
            .await?;

        self.update_context(AuthContext::Authenticated(Authentication {
            access_token: response.access,
            refresh_token: response.refresh,
        }))?;

        Ok(())
    }

    async fn logout(&mut self) -> Result<(), Error> {
        self.update_context(AuthContext::NotAuthenticated {
            reason: Reason::Logout,
        })?;

        Ok(())
    }

    async fn refresh(&mut self) -> Result<(), Error> {
        if let Some(refresh_token) = self.context.access_token() {
            let body = RefreshRequest {
                refresh: String::from(refresh_token),
            };

            let response = self
                .request::<RefreshRequest, RefreshResponse>(
                    reqwest::Method::POST,
                    REFRESH_ROUTE,
                    body,
                )
                .await;

            match response {
                Ok(response) => {
                    self.update_context(AuthContext::Authenticated(Authentication {
                        access_token: response.access,
                        refresh_token: String::from(refresh_token),
                    }))?;

                    Ok(())
                }
                Err(_) => {
                    self.update_context(AuthContext::NotAuthenticated {
                        reason: Reason::Expired,
                    })?;

                    Err(Error::Unauthorized)
                }
            }
        } else {
            Err(Error::Unauthorized)
        }
    }

    fn update_context(&mut self, context: AuthContext) -> Result<(), Error> {
        state::set_into_store(state::STORAGE_KEY_AUTH, context.clone())?;

        self.callback.emit(context.clone());
        self.context = context;

        Ok(())
    }

    async fn request<B, T>(&self, method: reqwest::Method, route: &str, body: B) -> Result<T, Error>
    where
        T: DeserializeOwned + 'static + std::fmt::Debug,
        B: Serialize + std::fmt::Debug,
    {
        let request = self.make_request::<B, T>(method, route, body).await?;
        self.send_request::<T>(request).await
    }

    async fn make_request<B, T>(
        &self,
        method: reqwest::Method,
        route: &str,
        body: B,
    ) -> Result<RequestBuilder, Error>
    where
        T: DeserializeOwned + 'static + std::fmt::Debug,
        B: Serialize + std::fmt::Debug,
    {
        let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;

        let root = self
            .config
            .as_ref()
            .ok_or(Error::NotInitialized)?
            .api_root
            .clone();

        let url = self
            .config
            .as_ref()
            .ok_or(Error::NotInitialized)?
            .api_root
            .join(route)
            .map_err(|err| Error::ConfigurationError(err.to_string()))?
            .clone();

        let api_token = self
            .config
            .as_ref()
            .ok_or(Error::NotInitialized)?
            .api_token
            .clone();

        log::debug!("root {root:?}");
        log::debug!("url {url:?}");

        let mut builder = reqwest::Client::new()
            .request(method, url)
            .header(reqwest::header::CONTENT_TYPE, "application/json");

        if let Some(token) = &api_token {
            builder = builder.header(HeaderName::from_static("api-key"), token);
        }

        if let Some(access_token) = self.context.access_token() {
            builder = builder.bearer_auth(access_token);
        }

        if allow_body {
            builder = builder.json(&body);
        }

        Ok(builder)
    }

    async fn send_request<T>(&self, request: RequestBuilder) -> Result<T, Error>
    where
        T: DeserializeOwned + 'static + std::fmt::Debug,
    {
        let response = request.send().await;

        if let Ok(data) = response {
            if data.status().is_success() {
                let data: Result<T, _> = data.json::<T>().await;
                if let Ok(data) = data {
                    Ok(data)
                } else {
                    Err(Error::DeserializeError)
                }
            } else {
                match data.status().as_u16() {
                    401 => Err(Error::Unauthorized),
                    403 => Err(Error::Forbidden),
                    404 => Err(Error::NotFound),
                    500 => Err(Error::InternalServerError),
                    422 => {
                        let data = data.json::<ErrorResponse>().await;
                        if let Ok(data) = data {
                            Err(Error::UnprocessableEntity(data))
                        } else {
                            Err(Error::DeserializeError)
                        }
                    }
                    _ => Err(Error::RequestError),
                }
            }
        } else {
            Err(Error::RequestError)
        }
    }
}

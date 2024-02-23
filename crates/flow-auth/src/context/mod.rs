mod agent;

pub use agent::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Authentication {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    NewSession,
    Expired,
    Logout,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AuthContext {
    NotInitialized,
    NotAuthenticated { reason: Reason },
    Authenticated(Authentication),
    Failed(String),
}

impl AuthContext {
    pub fn authentication(&self) -> Option<&Authentication> {
        match self {
            Self::Authenticated(auth) => Some(auth),
            _ => None,
        }
    }

    pub fn access_token(&self) -> Option<&str> {
        self.authentication().map(|auth| auth.access_token.as_str())
    }
}

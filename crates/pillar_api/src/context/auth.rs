use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Authentication {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    NewSession,
    Expired,
    Logout,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AuthContext {
    NotInitialized,
    NotAuthenticated { reason: Reason },
    Authenticated(Authentication),
}

impl AuthContext {
    pub fn authentication(&self) -> Option<&Authentication> {
        match self {
            Self::Authenticated(auth) => Some(auth),
            _ => None,
        }
    }

    pub fn refresh_token(&self) -> Option<&str> {
        self.authentication().map(|auth| auth.access_token.as_str())
    }

    pub fn access_token(&self) -> Option<&str> {
        self.authentication()
            .map(|auth| auth.refresh_token.as_str())
    }
}

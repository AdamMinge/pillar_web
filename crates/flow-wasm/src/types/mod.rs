mod auth;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use auth::{
    LoginInfo, LoginInfoWrapper, RegisterInfo, RegisterInfoWrapper, UserInfo, UserInfoWrapper,
    UserUpdateInfo, UserUpdateInfoWrapper,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type DeleteWrapper = HashMap<(), ()>;

mod user;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use user::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub errors: HashMap<String, Vec<String>>,
}

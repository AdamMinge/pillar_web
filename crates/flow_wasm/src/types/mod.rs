mod user;

pub use user::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub errors: HashMap<String, Vec<String>>,
}

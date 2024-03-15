use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub url: Url,
    pub username: String,
    pub email: String,
    pub verified: bool,
    pub activated: bool,
    pub staff: bool,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

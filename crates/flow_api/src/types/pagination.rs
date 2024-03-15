use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LimitOffset<T> {
    pub count: i32,
    pub next: Url,
    pub previous: Url,
    pub results: Vec<T>,
}

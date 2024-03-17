use crate::{api, types};

use uuid::Uuid;

pub async fn users(
    client: &mut api::Client,
) -> Result<types::LimitOffset<types::User>, api::Error> {
    client.get("user/").await
}

pub async fn user(client: &mut api::Client, id: Uuid) -> Result<types::User, api::Error> {
    client.get(&format!("user/{}/", id)).await
}

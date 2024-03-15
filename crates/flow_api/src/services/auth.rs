use crate::{api, types};

pub async fn signup(
    client: &mut api::Client,
    body: types::Signup,
) -> Result<types::User, api::Error> {
    client.post("auth/signup/", body).await
}

pub async fn activation(client: &mut api::Client, body: types::Token) -> Result<(), api::Error> {
    client.post("auth/activation/", body).await
}

pub async fn resend_activation(
    client: &mut api::Client,
    body: types::Email,
) -> Result<(), api::Error> {
    client.post("auth/activation/resend/", body).await
}

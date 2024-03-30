use crate::{api, types};

pub async fn signup(
    client: &mut api::Client,
    signup: types::Signup,
) -> Result<types::User, api::Error> {
    client.post("auth/signup/", signup).await
}

pub async fn send_activation(
    client: &mut api::Client,
    sender: types::EmailSender,
) -> Result<(), api::Error> {
    client.post("auth/send_activation/", sender).await
}

pub async fn activation(
    client: &mut api::Client,
    email: types::Email,
    token: types::Token,
) -> Result<(), api::Error> {
    client
        .post(&format!("auth/activation/{}/", token.token), email)
        .await
}

pub async fn send_recovery(
    client: &mut api::Client,
    sender: types::EmailSender,
) -> Result<(), api::Error> {
    client.post("auth/send_recovery/", sender).await
}

pub async fn recovery(
    client: &mut api::Client,
    email: types::Email,
    token: types::Token,
) -> Result<(), api::Error> {
    client
        .post(&format!("auth/recovery/{}/", token.token), email)
        .await
}

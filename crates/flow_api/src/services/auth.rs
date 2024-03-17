use crate::{api, types};

pub async fn signup(
    client: &mut api::Client,
    signup: types::Signup,
) -> Result<types::User, api::Error> {
    client.post("auth/signup/", signup).await
}

pub async fn activation(client: &mut api::Client, token: types::Token) -> Result<(), api::Error> {
    client.post("auth/activation/", token).await
}

pub async fn resend_activation(
    client: &mut api::Client,
    email: types::Email,
) -> Result<(), api::Error> {
    client.post("auth/activation/resend/", email).await
}

use uuid::Uuid;

use super::requests::*;
use crate::error::*;
use crate::types::*;

pub async fn create_user(create_user: CreateUser) -> Result<UserWrapper, Error> {
    request_post::<CreateUser, UserWrapper>("/auth/signup".to_string(), create_user).await
}

pub async fn activate_user(activate_user: ActivateUserWrapper) -> Result<(), Error> {
    request_post::<ActivateUserWrapper, ()>("/auth/signup/activation".to_string(), activate_user)
        .await
}

pub async fn users() -> Result<UserWrapper, Error> {
    request_get::<UserWrapper>("/user".to_string()).await
}

pub async fn user(uuid: Uuid) -> Result<UserWrapper, Error> {
    request_get::<UserWrapper>(format!("/user/{uuid}")).await
}

pub async fn current_user() -> Result<UserWrapper, Error> {
    user(Uuid::nil()).await
}

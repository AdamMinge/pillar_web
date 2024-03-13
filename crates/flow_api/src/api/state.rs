use super::Error;
use gloo_storage::errors::StorageError;
use gloo_storage::{SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub(crate) const STORAGE_KEY_AUTH: &str = "flow/auth";

pub(crate) fn get_from_store<T, K>(key: K) -> Result<Option<T>, Error>
where
    T: for<'de> Deserialize<'de>,
    K: AsRef<str> + Display,
{
    match SessionStorage::get::<T>(key.as_ref()) {
        Err(StorageError::KeyNotFound(_)) => Ok(None),
        Err(err) => Err(Error::StorageError(err.to_string())),
        Ok(value) => Ok(Some(value)),
    }
}

pub(crate) fn set_into_store<T, K>(key: K, value: T) -> Result<(), Error>
where
    T: Serialize,
    K: AsRef<str> + Display,
{
    match SessionStorage::set::<T>(key.as_ref(), value) {
        Err(err) => Err(Error::StorageError(err.to_string())),
        Ok(_) => Ok(()),
    }
}

use std::fmt;
use std::ops::Deref;

use yew::prelude::*;

use crate::services::set_token;
use crate::types::UserInfo;

pub struct UseUserContextHandle {
    inner: UseStateHandle<UserInfo>,
}

impl UseUserContextHandle {
    pub fn login(&self, value: UserInfo) {
        set_token(Some(value.token.clone()));
        self.inner.set(value);
    }

    pub fn logout(&self) {
        set_token(None);
        self.inner.set(UserInfo::default());
    }
}

impl Deref for UseUserContextHandle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl fmt::Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

#[hook]
pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<UserInfo>>().unwrap();

    UseUserContextHandle { inner }
}

use crate::context::AuthContext;
use yew::prelude::*;

#[hook]
pub fn use_auth() -> Option<AuthContext> {
    use_context()
}

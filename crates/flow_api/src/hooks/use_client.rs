use crate::context::ClientContext;
use yew::prelude::*;

#[hook]
pub fn use_client() -> Option<ClientContext> {
    use_context()
}

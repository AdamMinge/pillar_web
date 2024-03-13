use crate::api::Client;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Debug)]
pub struct ClientContext(Client, usize);

static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl ClientContext {
    pub fn new(client: Client) -> Self {
        let id = COUNTER.fetch_add(1, Ordering::AcqRel);

        Self(client, id)
    }
}

impl PartialEq for ClientContext {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl Deref for ClientContext {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ClientContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

use crate::agent;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Debug)]
pub struct AgentContext(agent::Agent, usize);

static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl AgentContext {
    pub fn new(agent: agent::Agent) -> Self {
        let id = COUNTER.fetch_add(1, Ordering::AcqRel);

        Self(agent, id)
    }
}

impl PartialEq for AgentContext {
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl Deref for AgentContext {
    type Target = agent::Agent;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AgentContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

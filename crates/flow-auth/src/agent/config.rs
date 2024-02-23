use std::time::Duration;

#[derive(Clone, Debug)]
pub struct AgentConfiguration {
    pub grace_period: Duration,
}

impl PartialEq for AgentConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.grace_period == other.grace_period
    }
}

impl Eq for AgentConfiguration {}

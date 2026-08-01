use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Healthy,
    Degraded { reason: String },
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub status: Status,
}

#[derive(Debug, Clone)]
pub struct Incident {
    pub id: u64,
    pub service: String,
    pub summary: String,
}

impl PartialEq for Incident {
    fn eq(&self, other: &Self) -> bool {
        self.service == other.service && self.summary == other.summary
    }
}

impl Eq for Incident {}

impl Hash for Incident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.service.hash(state);
        self.summary.hash(state);
    }
}

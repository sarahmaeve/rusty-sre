use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeliveryPolicy {
    pub retries: u8,
    #[serde(default, skip_serializing_if = "is_false")]
    pub cancel_on_error: bool,
}

const fn is_false(value: &bool) -> bool {
    !*value
}

pub fn serialize_policy(policy: &DeliveryPolicy) -> serde_json::Result<String> {
    serde_json::to_string(policy)
}

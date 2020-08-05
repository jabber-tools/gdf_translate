use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// type custom_payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GACustomPayloadType {
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub payload: JsonValue,
}

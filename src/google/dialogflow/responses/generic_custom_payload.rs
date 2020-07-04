use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// platform is number (all other channels supporting custom payload)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenericCustomPayloadType {
    #[serde(rename = "type")]
    pub message_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub payload: JsonValue,
}

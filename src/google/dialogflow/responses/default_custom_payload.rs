use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// no platform specified (DEFAULT CHANNEL)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DefaultCustomPayloadType {
    #[serde(rename = "type")]
    pub message_type: u8,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub payload: JsonValue,
}

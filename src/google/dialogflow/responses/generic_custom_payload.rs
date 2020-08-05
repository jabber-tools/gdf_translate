use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// type 4
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenericCustomPayloadType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub payload: JsonValue,
    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

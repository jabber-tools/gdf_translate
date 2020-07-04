use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenericImageResponseType {
    #[serde(rename = "type")]
    pub message_type: u8,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

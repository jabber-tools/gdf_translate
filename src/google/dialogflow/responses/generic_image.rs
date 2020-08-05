use serde::{Deserialize, Serialize};

// type 3
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenericImageResponseType {
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech: Option<String>,
}

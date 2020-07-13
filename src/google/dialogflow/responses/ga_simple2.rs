use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GASimpleResponseType2 {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssml: Option<String>,
    #[serde(rename = "displayText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
}

impl Translate for GASimpleResponseType2 {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        if let Some(text_to_speech) = &self.text_to_speech {
            map_to_translate.insert(format!("{:p}", text_to_speech), text_to_speech.to_owned());
        }

        if let Some(ssml) = &self.ssml {
            map_to_translate.insert(format!("{:p}", ssml), ssml.to_owned());
        }

        if let Some(display_text) = &self.display_text {
            map_to_translate.insert(format!("{:p}", display_text), display_text.to_owned());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        if let Some(text_to_speech) = &self.text_to_speech {
            self.text_to_speech = Some(
                translations_map
                    .get(&format!("{:p}", text_to_speech))
                    .unwrap()
                    .to_owned(),
            );
        }

        if let Some(ssml) = &self.ssml {
            self.ssml = Some(
                translations_map
                    .get(&format!("{:p}", ssml))
                    .unwrap()
                    .to_owned(),
            );
        }

        if let Some(display_text) = &self.display_text {
            self.display_text = Some(
                translations_map
                    .get(&format!("{:p}", display_text))
                    .unwrap()
                    .to_owned(),
            );
        }
    }
}

use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GASimpleResponseItem {
    #[serde(rename = "textToSpeech")]
    pub text_to_speech: String,
    pub ssml: String,
    #[serde(rename = "displayText")]
    pub display_text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GASimpleResponseType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub items: Vec<GASimpleResponseItem>,
}

impl Translate for GASimpleResponseType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        for item in self.items.iter() {
            map_to_translate.insert(
                format!("{:p}", &item.text_to_speech),
                item.text_to_speech.to_owned(),
            );
            map_to_translate.insert(format!("{:p}", &item.ssml), item.ssml.to_owned());
            map_to_translate.insert(
                format!("{:p}", &item.display_text),
                item.display_text.to_owned(),
            );
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        for item in self.items.iter_mut() {
            item.text_to_speech = translations_map
                .get(&format!("{:p}", &item.text_to_speech))
                .unwrap()
                .to_owned();

            item.ssml = translations_map
                .get(&format!("{:p}", &item.ssml))
                .unwrap()
                .to_owned();

            item.display_text = translations_map
                .get(&format!("{:p}", &item.display_text))
                .unwrap()
                .to_owned();
        }
    }
}

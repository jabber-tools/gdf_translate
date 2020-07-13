use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GALinkOutSuggestionType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "destinationName")]
    pub destination_name: String,
    pub url: String,
}

impl Translate for GALinkOutSuggestionType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(
            format!("{:p}", &self.destination_name),
            self.destination_name.to_owned(),
        );

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.destination_name = translations_map
            .get(&format!("{:p}", &self.destination_name))
            .unwrap()
            .to_owned();
    }
}

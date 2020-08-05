use crate::google::dialogflow::responses::ga_item::GAItem;
use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

// type list_card
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GAListType {
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub title: String,
    pub subtitle: String,
    pub items: Vec<GAItem>,
    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech: Option<String>,
}

impl Translate for GAListType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());
        map_to_translate.insert(format!("{:p}", &self.subtitle), self.subtitle.to_owned());

        for item in self.items.iter() {
            map_to_translate.extend(item.to_translation());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();

        self.subtitle = translations_map
            .get(&format!("{:p}", &self.subtitle))
            .unwrap()
            .to_owned();

        for item in self.items.iter_mut() {
            item.from_translation(translations_map);
        }
    }
}

use crate::google::dialogflow::agent::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GACardTypeButton {
    pub title: String,
    #[serde(rename = "openUrlAction")]
    pub open_url_action: GAOpenUrlAction,
}

impl Translate for GACardTypeButton {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GAOpenUrlAction {
    pub url: String,
    #[serde(rename = "urlTypeHint")]
    pub url_type_hint: String,
}

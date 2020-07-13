use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GAImage {
    pub url: String,
    #[serde(rename = "accessibilityText")]
    pub accessibility_text: String,
}

impl Translate for GAImage {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(
            format!("{:p}", &self.accessibility_text),
            self.accessibility_text.to_owned(),
        );

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.accessibility_text = translations_map
            .get(&format!("{:p}", &self.accessibility_text))
            .unwrap()
            .to_owned();
    }
}

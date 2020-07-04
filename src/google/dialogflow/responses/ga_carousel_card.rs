use crate::gdf_agent::Translate;
use crate::google::dialogflow::responses::ga_item::GAItem;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GACarouselCardType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub items: Vec<GAItem>,
}

impl Translate for GACarouselCardType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        for item in self.items.iter() {
            map_to_translate.extend(item.to_translation());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        for item in self.items.iter_mut() {
            item.from_translation(translations_map);
        }
    }
}

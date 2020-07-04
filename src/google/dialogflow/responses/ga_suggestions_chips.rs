use crate::gdf_agent::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GASuggestionChipsTypeSuggestion {
    pub title: String,
}

impl Translate for GASuggestionChipsTypeSuggestion {
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
pub struct GASuggestionChipsType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub suggestions: Vec<GASuggestionChipsTypeSuggestion>,
}

impl Translate for GASuggestionChipsType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        for suggestion in self.suggestions.iter() {
            map_to_translate.extend(suggestion.to_translation());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        for suggestion in self.suggestions.iter_mut() {
            suggestion.from_translation(translations_map);
        }
    }
}

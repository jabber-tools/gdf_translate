use crate::google::dialogflow::agent::{Translate, RE_ENTITY_ENTRY_FILE};
use regex::Captures;
use serde::{Deserialize, Serialize};
use std::collections;

// see https://serde.rs/field-attrs.html
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub id: String,
    pub name: String,

    #[serde(rename = "isOverridable")]
    pub is_overridable: bool,

    #[serde(rename = "isEnum")]
    pub is_enum: bool,

    #[serde(rename = "isRegexp")]
    pub is_regexp: bool,

    #[serde(rename = "automatedExpansion")]
    pub automated_expansion: bool,

    #[serde(rename = "allowFuzzyExtraction")]
    pub allow_fuzzy_extraction: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EntityEntry {
    pub value: String,
    pub synonyms: Vec<String>,
}

impl Translate for EntityEntry {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.value), self.value.to_owned());

        for synonym in self.synonyms.iter() {
            map_to_translate.insert(format!("{:p}", synonym), synonym.to_owned());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        if let Some(val) = translations_map.get(&format!("{:p}", &self.value)) {
            self.value = val.to_owned();
        }

        for synonym in self.synonyms.iter_mut() {
            if let Some(syn) = translations_map.get(&format!("{:p}", synonym)) {
                *synonym = syn.to_owned();
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityFile {
    pub file_name: String,
    pub file_content: Entity,
}

impl EntityFile {
    pub fn new(file_name: String, file_content: Entity) -> Self {
        EntityFile {
            file_name,
            file_content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EntityEntriesFile {
    pub file_name: String,
    pub file_content: Vec<EntityEntry>,
}

impl EntityEntriesFile {
    pub fn new(file_name: String, file_content: Vec<EntityEntry>) -> Self {
        EntityEntriesFile {
            file_name,
            file_content,
        }
    }

    pub fn to_new_language(&self, new_lang_code: &str) -> Self {
        let mut cloned = self.clone();
        cloned.file_name = RE_ENTITY_ENTRY_FILE
            .replace(&self.file_name, |caps: &Captures| {
                format!("{}{}{}", &caps[1], new_lang_code, ".json")
            })
            .to_string();

        cloned
    }
}

use crate::google::dialogflow::agent::{Translate, RE_INTENT_UTTERANCE_FILE};
use regex::Captures;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IntentUtteranceData {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,
    #[serde(rename = "userDefined")]
    pub user_defined: bool,
}

impl Translate for IntentUtteranceData {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();
        map_to_translate.insert(format!("{:p}", &self.text), self.text.to_owned());
        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        if let Some(text) = translations_map.get(&format!("{:p}", &self.text)) {
            self.text = text.to_owned()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntentUtterance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub data: Vec<IntentUtteranceData>,

    #[serde(rename = "isTemplate")]
    pub is_template: bool,
    pub count: i8,
    pub updated: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntentUtterancesFile {
    pub file_name: String,
    pub file_content: Vec<IntentUtterance>,
}

impl IntentUtterancesFile {
    pub fn new(file_name: String, file_content: Vec<IntentUtterance>) -> Self {
        IntentUtterancesFile {
            file_name,
            file_content,
        }
    }

    pub fn to_new_language(&self, new_lang_code: &str) -> Self {
        let mut cloned = self.clone();
        cloned.file_name = RE_INTENT_UTTERANCE_FILE
            .replace(&self.file_name, |caps: &Captures| {
                format!("{}{}{}", &caps[1], new_lang_code, ".json")
            })
            .to_string();

        for file_content in cloned.file_content.iter_mut() {
            file_content.lang = Some(new_lang_code.to_owned());
        }

        cloned
    }
}

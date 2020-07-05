use crate::google::dialogflow::agent::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum StringOrVecOfString {
    Str(String),
    StrArray(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GenericTextResponseType {
    #[serde(rename = "type")]
    pub message_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub speech: StringOrVecOfString,
}

impl Translate for GenericTextResponseType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();
        match &self.speech {
            StringOrVecOfString::Str(str_val) => {
                map_to_translate.insert(format!("{:p}", str_val), str_val.to_owned());
            }
            StringOrVecOfString::StrArray(str_vec) => {
                for item in str_vec.iter() {
                    map_to_translate.insert(format!("{:p}", item), item.to_owned());
                }
            }
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        match &self.speech {
            StringOrVecOfString::Str(str_val) => {
                self.speech = StringOrVecOfString::Str(
                    translations_map
                        .get(&format!("{:p}", str_val))
                        .unwrap()
                        .to_owned(),
                );
            }
            StringOrVecOfString::StrArray(str_vec) => {
                let mut speech_vec = vec![];
                for item in str_vec.iter() {
                    speech_vec.push(
                        translations_map
                            .get(&format!("{:p}", item))
                            .unwrap()
                            .to_owned(),
                    );
                }
                self.speech = StringOrVecOfString::StrArray(speech_vec);
            }
        }
    }
}

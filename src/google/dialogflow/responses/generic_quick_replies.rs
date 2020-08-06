use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

// type 2
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GenericQuickRepliesResponseType {
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub title: String,
    pub replies: Vec<String>,
    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech: Option<String>,
}

impl Translate for GenericQuickRepliesResponseType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());

        for reply in self.replies.iter() {
            map_to_translate.insert(format!("{:p}", reply), reply.to_owned());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();

        for reply in self.replies.iter_mut() {
            *reply = translations_map
                .get(&format!("{:p}", reply))
                .unwrap()
                .to_owned();
        }

        // GDF allows only 20 chars per quick reply, translation must be truncated accordingly!
        for reply in self.replies.iter_mut() {
            *reply = substring(reply.to_string(), 0, 20);
        }
    }
}

// to prevent errors like panicked at 'byte index 20 is out of bounds of ...
fn substring(full_string: String, start: usize, len: usize) -> String {
    full_string.chars().skip(start).take(len).collect()
}

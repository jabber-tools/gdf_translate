use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GenericQuickRepliesResponseType {
    #[serde(rename = "type")]
    pub message_type: u8,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub title: String,
    pub replies: Vec<String>,
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
    }
}

use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

// this is not standalone type, it is always contained in other types, e.g. in carousel_card type etc.
// all params are options, unfortunatelly we have seen messages like this (see "image": {}):
/*
{
    "type": "basic_card",
    "platform": "google",
    "title": "",
    "buttons": [
      {
        "openUrlAction": {
          "url": "https://someurl.com",
          "urlTypeHint": "URL_TYPE_HINT_UNSPECIFIED"
        },
        "title": "https://someurl.com"
      }
    ],
    "textToSpeech": "",
    "formattedText": "Hello+",
    "image": {},
    "lang": "en",
    "condition": ""
  }
*/
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GAImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "accessibilityText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech: Option<String>,
}

impl Translate for GAImage {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        if let Some(accessibility_text) = &self.accessibility_text {
            map_to_translate.insert(
                format!("{:p}", accessibility_text),
                accessibility_text.to_owned(),
            );
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        if let Some(accessibility_text) = &self.accessibility_text {
            self.accessibility_text = Some(
                translations_map
                    .get(&format!("{:p}", accessibility_text))
                    .unwrap()
                    .to_owned(),
            );
        }
    }
}

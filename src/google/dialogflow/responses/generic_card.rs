use crate::google::dialogflow::agent::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GenericCardResponseButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback: Option<String>,
}

impl Translate for GenericCardResponseButton {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.text), self.text.to_owned());

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.text = translations_map
            .get(&format!("{:p}", &self.text))
            .unwrap()
            .to_owned();
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GenericCardResponseType {
    #[serde(rename = "type")]
    pub message_type: u8,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<GenericCardResponseButton>>,
}

impl Translate for GenericCardResponseType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());
        if let Some(subtitle) = &self.subtitle {
            map_to_translate.insert(format!("{:p}", subtitle), subtitle.to_owned());
        }

        if let Some(buttons) = &self.buttons {
            for button in buttons.iter() {
                map_to_translate.extend(button.to_translation());
            }
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();

        if let Some(subtitle) = &self.subtitle {
            self.subtitle = Some(
                translations_map
                    .get(&format!("{:p}", subtitle))
                    .unwrap()
                    .to_owned(),
            );
        }

        if let Some(buttons) = &mut self.buttons {
            for button in buttons.iter_mut() {
                button.from_translation(translations_map);
            }
        }
    }
}

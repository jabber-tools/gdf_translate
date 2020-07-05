use crate::google::dialogflow::agent::Translate;
use crate::google::dialogflow::responses::ga_image::GAImage;
use crate::google::dialogflow::responses::ga_shared::GACardTypeButton;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GABasicCardType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "formattedText")]
    pub formatted_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<GAImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<GACardTypeButton>>,
}

impl Translate for GABasicCardType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        if let Some(title) = &self.title {
            map_to_translate.insert(format!("{:p}", title), title.to_owned());
        }

        if let Some(subtitle) = &self.subtitle {
            map_to_translate.insert(format!("{:p}", subtitle), subtitle.to_owned());
        }

        map_to_translate.insert(
            format!("{:p}", &self.formatted_text),
            self.formatted_text.to_owned(),
        );

        if let Some(image) = &self.image {
            map_to_translate.extend(image.to_translation());
        }

        if let Some(buttons) = &self.buttons {
            for button in buttons.iter() {
                map_to_translate.extend(button.to_translation())
            }
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        if let Some(title) = &self.title {
            self.title = Some(
                translations_map
                    .get(&format!("{:p}", title))
                    .unwrap()
                    .to_owned(),
            );
        }

        if let Some(subtitle) = &self.subtitle {
            self.subtitle = Some(
                translations_map
                    .get(&format!("{:p}", subtitle))
                    .unwrap()
                    .to_owned(),
            );
        }

        self.formatted_text = translations_map
            .get(&format!("{:p}", &self.formatted_text))
            .unwrap()
            .to_owned();

        if let Some(image) = &mut self.image {
            image.from_translation(translations_map);
        }

        if let Some(buttons) = &mut self.buttons {
            for button in buttons.iter_mut() {
                button.from_translation(translations_map);
            }
        }
    }
}

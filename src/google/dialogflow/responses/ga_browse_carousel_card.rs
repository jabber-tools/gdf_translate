use crate::google::dialogflow::responses::ga_image::GAImage;
use crate::google::dialogflow::responses::ga_shared::GAOpenUrlAction;
use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GAItemBrowseCarousel {
    pub footer: String,
    #[serde(rename = "openUrlAction")]
    pub open_url_action: GAOpenUrlAction,
    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<GAImage>,
    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech: Option<String>,
}

impl Translate for GAItemBrowseCarousel {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.footer), self.footer.to_owned());
        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());
        map_to_translate.insert(
            format!("{:p}", &self.description),
            self.description.to_owned(),
        );

        if let Some(image) = &self.image {
            map_to_translate.extend(image.to_translation());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.footer = translations_map
            .get(&format!("{:p}", &self.footer))
            .unwrap()
            .to_owned();

        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();

        self.description = translations_map
            .get(&format!("{:p}", &self.description))
            .unwrap()
            .to_owned();

        if let Some(image) = &mut self.image {
            image.from_translation(translations_map);
        }
    }
}

// type browse_carousel_card
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GABrowseCarouselCardType {
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub items: Vec<GAItemBrowseCarousel>,
}

impl Translate for GABrowseCarouselCardType {
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

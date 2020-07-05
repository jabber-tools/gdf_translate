use crate::google::dialogflow::agent::Translate;
use crate::google::dialogflow::responses::ga_image::GAImage;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GAMediaObject {
    name: String,
    description: String,
    #[serde(rename = "largeImage")]
    large_image: GAImage,
    #[serde(rename = "contentUrl")]
    content_url: String,
}

impl Translate for GAMediaObject {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.name), self.name.to_owned());
        map_to_translate.insert(
            format!("{:p}", &self.description),
            self.description.to_owned(),
        );
        map_to_translate.extend(self.large_image.to_translation());

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.name = translations_map
            .get(&format!("{:p}", &self.name))
            .unwrap()
            .to_owned();

        self.description = translations_map
            .get(&format!("{:p}", &self.description))
            .unwrap()
            .to_owned();

        self.large_image.from_translation(translations_map);
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GAMediaContentType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "mediaObjects")]
    pub media_objects: Vec<GAMediaObject>,
}

impl Translate for GAMediaContentType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        for media_object in self.media_objects.iter() {
            map_to_translate.extend(media_object.to_translation());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        for media_object in self.media_objects.iter_mut() {
            media_object.from_translation(translations_map);
        }
    }
}

use crate::google::dialogflow::responses::ga_image::GAImage;
use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GAListTypeItemOptionInfo {
    key: String,
    synonyms: Vec<String>,
}

impl Translate for GAListTypeItemOptionInfo {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.key), self.key.to_owned());

        for synonym in self.synonyms.iter() {
            map_to_translate.insert(format!("{:p}", synonym), synonym.to_owned());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.key = translations_map
            .get(&format!("{:p}", &self.key))
            .unwrap()
            .to_owned();

        for synonym in self.synonyms.iter_mut() {
            *synonym = translations_map
                .get(&format!("{:p}", synonym))
                .unwrap()
                .to_owned();
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GAItem {
    #[serde(rename = "optionInfo")]
    pub option_info: GAListTypeItemOptionInfo,
    pub title: String,
    pub description: String,
    pub image: GAImage,
}

impl Translate for GAItem {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());
        map_to_translate.insert(
            format!("{:p}", &self.description),
            self.description.to_owned(),
        );
        map_to_translate.extend(self.option_info.to_translation());
        map_to_translate.extend(self.image.to_translation());

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();

        self.description = translations_map
            .get(&format!("{:p}", &self.description))
            .unwrap()
            .to_owned();

        self.option_info.from_translation(translations_map);
        self.image.from_translation(translations_map);
    }
}

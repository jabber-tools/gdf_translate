use crate::google::dialogflow::responses::ga_shared::GACardTypeButton;
use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GATableCardRowCell {
    pub text: String,
}

impl Translate for GATableCardRowCell {
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
pub struct GATableCardRow {
    pub cells: Vec<GATableCardRowCell>,
    #[serde(rename = "dividerAfter")]
    pub divider_after: bool,
}

impl Translate for GATableCardRow {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        for cell in self.cells.iter() {
            map_to_translate.extend(cell.to_translation());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        for cell in self.cells.iter_mut() {
            cell.from_translation(translations_map);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GATableCardType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "columnProperties")]
    pub column_properties: Vec<std::collections::HashMap<String, String>>,
    pub rows: Vec<GATableCardRow>,
    pub buttons: Vec<GACardTypeButton>,
}

impl Translate for GATableCardType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());
        map_to_translate.insert(format!("{:p}", &self.subtitle), self.subtitle.to_owned());

        for row in self.rows.iter() {
            map_to_translate.extend(row.to_translation());
        }

        for button in self.buttons.iter() {
            map_to_translate.extend(button.to_translation());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();

        self.subtitle = translations_map
            .get(&format!("{:p}", &self.subtitle))
            .unwrap()
            .to_owned();

        for row in self.rows.iter_mut() {
            row.from_translation(translations_map);
        }

        for button in self.buttons.iter_mut() {
            button.from_translation(translations_map);
        }
    }
}

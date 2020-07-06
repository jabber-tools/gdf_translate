use crate::google::dialogflow::agent::Translate;
use crate::google::dialogflow::responses::ga_shared::StringOrVecOfString;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    // see prebuilt agent Online Shopping, intent delivery.options
    // somehow they managed to define Card on default channel ->
    // mandatory platform was changed to optional platform, seems to be working fine
    // pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<GenericCardResponseButton>>,

    // card cannot have really speech but after title was converted to Option
    // GenericTextResponseType got confused with GenericCardResponseType (i.e. text response identified as generic card response)
    // and we lost speech during serialization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech: Option<StringOrVecOfString>,
}

impl Translate for GenericCardResponseType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        if let Some(title) = &self.title {
            map_to_translate.insert(format!("{:p}", title), title.to_owned());
        }

        if let Some(subtitle) = &self.subtitle {
            map_to_translate.insert(format!("{:p}", subtitle), subtitle.to_owned());
        }

        if let Some(buttons) = &self.buttons {
            for button in buttons.iter() {
                map_to_translate.extend(button.to_translation());
            }
        }

        if let Some(speech) = &self.speech {
            match speech {
                StringOrVecOfString::Str(str_val) => {
                    map_to_translate.insert(format!("{:p}", str_val), str_val.to_owned());
                }
                StringOrVecOfString::StrArray(str_vec) => {
                    for item in str_vec.iter() {
                        map_to_translate.insert(format!("{:p}", item), item.to_owned());
                    }
                }
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

        if let Some(buttons) = &mut self.buttons {
            for button in buttons.iter_mut() {
                button.from_translation(translations_map);
            }
        }

        if let Some(speech) = &self.speech {
            match speech {
                StringOrVecOfString::Str(str_val) => {
                    self.speech = Some(StringOrVecOfString::Str(
                        translations_map
                            .get(&format!("{:p}", str_val))
                            .unwrap()
                            .to_owned(),
                    ));
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
                    self.speech = Some(StringOrVecOfString::StrArray(speech_vec));
                }
            }
        }
    }
}

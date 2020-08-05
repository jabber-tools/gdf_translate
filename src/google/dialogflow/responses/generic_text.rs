use crate::google::dialogflow::responses::ga_shared::StringOrVecOfString;
use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

// type 0
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GenericTextResponseType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech: Option<StringOrVecOfString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech: Option<String>,
}

impl Translate for GenericTextResponseType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Result;
    #[allow(unused_imports)]
    use crate::google::dialogflow::responses::{normalize_json, MessageType};
    use assert_json_diff::assert_json_eq;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Messages {
        pub messages: Vec<MessageType>,
    }

    #[test]
    fn test_generic_text_1() -> Result<()> {
        // type is string
        let default_text_response_1 = r#"
        {
            "type": "0",
            "lang": "fr",
            "speech": [
              "Vous pouvez contacter notre équipe de support technique au 1-855-345-7447"
            ],
            "condition": ""
          }
        "#;

        // title + textToSpeech probably for future use
        let default_text_response_2 = r#"
        {
            "type": "0",
            "title": "",
            "textToSpeech": "",
            "lang": "fr",
            "speech": [
              "Vous pouvez contacter notre équipe de support technique au 1-855-345-7447"
            ],
            "condition": ""
          }
        "#;

        let default_text_response_3 = r#"
        {
            "type": "0",
            "title": "",
            "textToSpeech": "",
            "lang": "en",
            "speech": [
              "You can contact our Technical Support team on 1-855-345-7447"
            ],
            "condition": ""
          }
        "#;

        let messages = format!(
            r#"
          {{
            "messages": [
            {default_text_response_1},
            {default_text_response_2},
            {default_text_response_3}
          ]
        }}
        "#,
            default_text_response_1 = default_text_response_1,
            default_text_response_2 = default_text_response_2,
            default_text_response_3 = default_text_response_3
        );

        println!("messages: {}", messages);

        let messages_struct: Messages = serde_json::from_str(&messages)?;
        println!("messages_struct {:#?}", messages_struct);

        let back_to_str = serde_json::to_string(&messages_struct)?;

        // assert_eq!(normalize_json(&messages), normalize_json(&back_to_str));
        assert_json_eq!(
            serde_json::from_str(&messages)?,
            serde_json::from_str(&back_to_str)?
        );

        Ok(())
    }
}

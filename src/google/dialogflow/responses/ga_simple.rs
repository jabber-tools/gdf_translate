use crate::google::gcloud::translate::Translate;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GASimpleResponseItem {
    #[serde(rename = "textToSpeech")]
    pub text_to_speech: String,
    pub ssml: String,
    #[serde(rename = "displayText")]
    pub display_text: String,
    // two attributes below seems to be added recently (seems to be alwasy set to ""). Adding
    // just so that serde can parse this message for now we are not supporting translation of these items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GASimpleResponseType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    // two attributes below seems to be added recently (seems to be alwasy set to ""). Adding
    // just so that serde can parse this message for now we are not supporting translation of these items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech: Option<String>,

    pub items: Vec<GASimpleResponseItem>,
}

impl Translate for GASimpleResponseType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        for item in self.items.iter() {
            map_to_translate.insert(
                format!("{:p}", &item.text_to_speech),
                item.text_to_speech.to_owned(),
            );
            map_to_translate.insert(format!("{:p}", &item.ssml), item.ssml.to_owned());
            map_to_translate.insert(
                format!("{:p}", &item.display_text),
                item.display_text.to_owned(),
            );
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        for item in self.items.iter_mut() {
            item.text_to_speech = translations_map
                .get(&format!("{:p}", &item.text_to_speech))
                .unwrap()
                .to_owned();

            item.ssml = translations_map
                .get(&format!("{:p}", &item.ssml))
                .unwrap()
                .to_owned();

            item.display_text = translations_map
                .get(&format!("{:p}", &item.display_text))
                .unwrap()
                .to_owned();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Result;
    use crate::google::dialogflow::responses::{normalize_json, MessageType};
    use assert_json_diff::assert_json_eq;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Messages {
        pub messages: Vec<MessageType>,
    }

    #[test]
    fn test_ga_simple_1() -> Result<()> {
        let simple_response_1 = r#"
    {
      "type": "simple_response",
      "platform": "google",
      "lang": "en",
      "condition": "",
      "items": [
        {
          "textToSpeech": "some speech",
          "ssml": "",
          "displayText": "some text"
        },
        {
          "textToSpeech": "some speech",
          "ssml": "",
          "displayText": "some text2"
        }
      ]
    }
    "#;

        let messages = format!(
            r#"
    {{
      "messages": [
       {simple_response_1}
      ]
     }}
    "#,
            simple_response_1 = simple_response_1,
        );

        println!("messages: {}", messages);

        let messages_struct: Messages = serde_json::from_str(&messages)?;
        println!("messages_struct {:#?}", messages_struct);

        let back_to_str = serde_json::to_string(&messages_struct)?;

        assert_eq!(normalize_json(&messages), normalize_json(&back_to_str));

        Ok(())
    }

    #[test]
    fn test_ga_simple_2() -> Result<()> {
        let simple_response_1 = r#"
        {
            "type": "simple_response",
            "platform": "google",
            "title": "",
            "textToSpeech": "",
            "items": [
              {
                "description": "",
                "textToSpeech": "Vous pouvez contacter notre équipe de support technique au 1-855-345-7447",
                "displayText": "",
                "footer": "",
                "ssml": ""
              }
            ],
            "lang": "fr",
            "condition": ""
          }
    "#;

        let messages = format!(
            r#"
    {{
      "messages": [
       {simple_response_1}
      ]
     }}
    "#,
            simple_response_1 = simple_response_1,
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

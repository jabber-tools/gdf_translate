/// This will replace first/draft implementation of IntentResponseMessage in gdf_agent
/// WIP
///
#[allow(unused_imports)]
use crate::errors::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultTextResponseType {
    #[serde(rename = "type")]
    pub message_type: u8,
    pub lang: String,
    pub condition: String,
    pub speech: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultCustomPayloadType {
    #[serde(rename = "type")]
    pub message_type: u8,
    pub lang: String,
    pub condition: String,
    pub payload: JsonValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GASimpleResponseItem {
    #[serde(rename = "textToSpeech")]
    pub text_to_speech: String,
    pub ssml: String,
    #[serde(rename = "displayText")]
    pub display_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GASimpleResponseType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    pub condition: String,
    pub items: Vec<GASimpleResponseItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GACustomPayloadType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    pub condition: String,
    pub payload: JsonValue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageType {
    DefaultTextResponse(DefaultTextResponseType),
    DefaultCustomPayload(DefaultCustomPayloadType),
    GASimpleResponse(GASimpleResponseType),
    GACustomPayload(GACustomPayloadType),
}

// removes all whitespaces and replaces some characters (as produced by serde serialization)
// with entities used by DialogFlow.
#[allow(dead_code)]
fn normalize_json(s: &str) -> String {
    let normalized_str: String = s.split_whitespace().collect();
    normalized_str
        .replace("\n", "")
        .replace("&", "\\u0026")
        .replace("'", "\\u0027")
        .replace("<", "\\u003c")
        .replace(">", "\\u003e")
        .replace("=", "\\u003d")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Messages {
        pub messages: Vec<MessageType>,
    }

    #[test]
    fn test_default_1() -> Result<()> {
        let default_text_response = r#"
        {
            "type": 0,
            "lang": "en",
            "condition": "",
            "speech": "Text response"
          }
        "#;

        let default_custom_payload = r#"
          {
            "type": 4,
            "lang": "en",
            "condition": "",
            "payload": {
              "foo": "custom payload"
            }
          }
        "#;

        let messages = format!(
            r#"
          {{
            "messages": [
            {default_text_response},
            {default_custom_payload}
          ]
        }}
        "#,
            default_text_response = default_text_response,
            default_custom_payload = default_custom_payload
        );

        println!("messages: {}", messages);

        let messages_struct: Messages = serde_json::from_str(&messages)?;
        println!("messages_struct {:#?}", messages_struct);

        let back_to_str = serde_json::to_string(&messages_struct)?;

        assert_eq!(normalize_json(&messages), normalize_json(&back_to_str));

        Ok(())
    }

    /* Google assistant */

    #[test]
    fn test_ga_1() -> Result<()> {
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

        let simple_response_2 = r#"
      {
        "type": "simple_response",
        "platform": "google",
        "lang": "en",
        "condition": "",
        "items": [
          {
            "textToSpeech": "111",
            "ssml": "",
            "displayText": ""
          },
          {
            "textToSpeech": "222 ga simple response",
            "ssml": "",
            "displayText": ""
          }
        ]
      }
      "#;

        let custom_payload_1 = r#"
      {
        "type": "custom_payload",
        "platform": "google",
        "lang": "en",
        "condition": "",
        "payload": {
          "google": {
            "foo": {
              "bar": {
                "foobar": "barfoo"
              }
            }
          }
        }
      }
      "#;

        let messages = format!(
            r#"
      {{
        "messages": [
         {simple_response_1},
         {simple_response_2},
          {custom_payload_1}
        ]
       }}
      "#,
            simple_response_1 = simple_response_1,
            simple_response_2 = simple_response_2,
            custom_payload_1 = custom_payload_1
        );

        println!("messages: {}", messages);

        let messages_struct: Messages = serde_json::from_str(&messages)?;
        println!("messages_struct {:#?}", messages_struct);

        let back_to_str = serde_json::to_string(&messages_struct)?;

        assert_eq!(normalize_json(&messages), normalize_json(&back_to_str));

        Ok(())
    }
}

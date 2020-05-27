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
    message_type: u8,
    lang: String,
    condition: String,
    speech: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultCustomPayloadType {
    #[serde(rename = "type")]
    message_type: u8,
    lang: String,
    condition: String,
    payload: JsonValue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageType {
    DefaultTextResponse(DefaultTextResponseType),
    DefaultCustomPayload(DefaultCustomPayloadType),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    messages: Vec<MessageType>,
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
}

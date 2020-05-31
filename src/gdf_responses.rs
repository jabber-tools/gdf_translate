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
pub struct GAImage {
    pub url: String,
    #[serde(rename = "accessibilityText")]
    pub accessibility_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GACardTypeButton {
    pub title: String,
    #[serde(rename = "openUrlAction")]
    pub open_url_action: GAOpenUrlAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GAOpenUrlAction {
    pub url: String,
    #[serde(rename = "urlTypeHint")]
    pub url_type_hint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GABasicCardType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    pub condition: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "formattedText")]
    pub formatted_text: String,
    pub image: GAImage,
    pub buttons: Vec<GACardTypeButton>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GAListTypeItemOptionInfo {
    key: String,
    synonyms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GAItem {
    #[serde(rename = "optionInfo")]
    pub option_info: GAListTypeItemOptionInfo,
    pub title: String,
    pub description: String,
    pub image: GAImage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GAItemBrowseCarousel {
    pub footer: String,
    #[serde(rename = "openUrlAction")]
    pub open_url_action: GAOpenUrlAction,
    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<GAImage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GAListType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    pub condition: String,
    pub title: String,
    pub subtitle: String,
    pub items: Vec<GAItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GASuggestionChipsTypeSuggestion {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GASuggestionChipsType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    pub condition: String,
    pub suggestions: Vec<GASuggestionChipsTypeSuggestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GALinkOutSuggestionType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    pub condition: String,
    #[serde(rename = "destinationName")]
    pub destination_name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GACarouselCardType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    pub condition: String,
    pub items: Vec<GAItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GABrowseCarouselCardType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    pub condition: String,
    pub items: Vec<GAItemBrowseCarousel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GAMediaObject {
    name: String,
    description: String,
    #[serde(rename = "largeImage")]
    large_image: GAImage,
    #[serde(rename = "contentUrl")]
    content_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GAMediaContentType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    pub condition: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "mediaObjects")]
    pub media_objects: Vec<GAMediaObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GATableCardRowCell {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GATableCardRow {
    pub cells: Vec<GATableCardRowCell>,
    #[serde(rename = "dividerAfter")]
    pub divider_after: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GATableCardType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    pub condition: String,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "columnProperties")]
    pub column_properties: Vec<std::collections::HashMap<String, String>>,
    pub rows: Vec<GATableCardRow>,
    pub buttons: Vec<GACardTypeButton>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageType {
    DefaultTextResponse(DefaultTextResponseType),
    DefaultCustomPayload(DefaultCustomPayloadType),
    GASimpleResponse(GASimpleResponseType),
    GACustomPayload(GACustomPayloadType),
    GABasicCard(GABasicCardType),
    GASuggestionChips(GASuggestionChipsType),
    GAList(GAListType),
    GALinkOutSuggestion(GALinkOutSuggestionType),
    GACarouselCard(GACarouselCardType),
    GABrowseCarouselCard(GABrowseCarouselCardType),
    GAMediaContent(GAMediaContentType),
    GATableCard(GATableCardType),
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
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

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

        let basic_card = r#"
        {
          "type": "basic_card",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "title": "title",
          "subtitle": "subtitle",
          "formattedText": "GA simple card",
          "image": {
            "url": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg",
            "accessibilityText": "accessbility text"
          },
          "buttons": [
            {
              "title": "weblink title",
              "openUrlAction": {
                "url": "https://github.com/contain-rs/linked-hash-map",
                "urlTypeHint": "URL_TYPE_HINT_UNSPECIFIED"
              }
            }
          ]
        }
      "#;

        let suggestions = r#"
      {
        "type": "suggestion_chips",
        "platform": "google",
        "lang": "en",
        "condition": "",
        "suggestions": [
          {
            "title": "chip1"
          },
          {
            "title": "chip2"
          }
        ]
      }
    "#;

        let list_card = r#"
    {
      "type": "list_card",
      "platform": "google",
      "lang": "en",
      "condition": "",
      "title": "list title",
      "subtitle": "",
      "items": [
        {
          "optionInfo": {
            "key": "key",
            "synonyms": []
          },
          "title": "item title",
          "description": "item desc",
          "image": {
            "url": "",
            "accessibilityText": ""
          }
        },
        {
          "optionInfo": {
            "key": "item2key",
            "synonyms": [
              "synonym2",
              "synonym22",
              "synionym222"
            ]
          },
          "title": "item title2",
          "description": "item desc2",
          "image": {
            "url": "",
            "accessibilityText": ""
          }
        }
      ]
    }
  "#;

        let linkout_suggestion = r#"
    {
      "type": "link_out_chip",
      "platform": "google",
      "lang": "en",
      "condition": "",
      "destinationName": "GA Link Out Suggestion",
      "url": "https://github.com/contain-rs/linked-hash-map"
    }    
    "#;

        let carousel_card = r#"
    {
      "type": "carousel_card",
      "platform": "google",
      "lang": "en",
      "condition": "",
      "items": [
        {
          "optionInfo": {
            "key": "key",
            "synonyms": [
              "syn",
              "sybn2"
            ]
          },
          "title": "item0",
          "description": "item0desc",
          "image": {
            "url": "",
            "accessibilityText": ""
          }
        },
        {
          "optionInfo": {
            "key": "key1",
            "synonyms": [
              "some syb"
            ]
          },
          "title": "item1",
          "description": "",
          "image": {
            "url": "",
            "accessibilityText": ""
          }
        }
      ]
    }        
    "#;

        let browse_carousel_card_1 = r#"
    {
      "type": "browse_carousel_card",
      "platform": "google",
      "lang": "en",
      "condition": "",
      "items": [
        {
          "footer": "footer",
          "openUrlAction": {
            "url": "https://www.idnes.cz/",
            "urlTypeHint": "URL_TYPE_HINT_UNSPECIFIED"
          },
          "title": "title",
          "description": "desc",
          "image": {
            "url": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg",
            "accessibilityText": "access text"
          }
        },
        {
          "footer": "",
          "openUrlAction": {
            "url": "https://www.idnes.cz/",
            "urlTypeHint": "AMP_CONTENT"
          },
          "title": "title2",
          "description": "",
          "image": {
            "url": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg",
            "accessibilityText": "acces text 2"
          }
        }
      ]
    }        
    "#;

        let browse_carousel_card_2 = r#"
    {
      "type": "browse_carousel_card",
      "platform": "google",
      "lang": "en",
      "condition": "",
      "items": [
        {
          "footer": "footer",
          "openUrlAction": {
            "url": "https://www.idnes.cz/",
            "urlTypeHint": "URL_TYPE_HINT_UNSPECIFIED"
          },
          "title": "title",
          "description": "desc"
        },
        {
          "footer": "",
          "openUrlAction": {
            "url": "https://www.idnes.cz/",
            "urlTypeHint": "AMP_CONTENT"
          },
          "title": "title2",
          "description": ""
        }
      ]
    }        
    "#;

        let media_content = r#"
    {
      "type": "media_content",
      "platform": "google",
      "lang": "en",
      "condition": "",
      "mediaType": "AUDIO",
      "mediaObjects": [
        {
          "name": "cad name",
          "description": "desc",
          "largeImage": {
            "url": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg",
            "accessibilityText": "acxcess text"
          },
          "contentUrl": "https://www.idnes.cz/"
        }
      ]
    }        
    "#;

        let messages = format!(
            r#"
      {{
        "messages": [
         {simple_response_1},
         {simple_response_2},
         {custom_payload_1},
         {basic_card},
         {suggestions},
         {list_card},
         {linkout_suggestion},
         {carousel_card},
         {browse_carousel_card_1},
         {browse_carousel_card_2},
         {media_content}
        ]
       }}
      "#,
            simple_response_1 = simple_response_1,
            simple_response_2 = simple_response_2,
            custom_payload_1 = custom_payload_1,
            basic_card = basic_card,
            suggestions = suggestions,
            list_card = list_card,
            linkout_suggestion = linkout_suggestion,
            carousel_card = carousel_card,
            browse_carousel_card_1 = browse_carousel_card_1,
            browse_carousel_card_2 = browse_carousel_card_2,
            media_content = media_content
        );

        println!("messages: {}", messages);

        let messages_struct: Messages = serde_json::from_str(&messages)?;
        println!("messages_struct {:#?}", messages_struct);

        let back_to_str = serde_json::to_string(&messages_struct)?;

        assert_eq!(normalize_json(&messages), normalize_json(&back_to_str));

        Ok(())
    }

    #[test]
    fn test_ga_2() -> Result<()> {
        let table_card = r#"
    {
      "type": "table_card",
      "platform": "google",
      "lang": "en",
      "condition": "",
      "title": "tit",
      "subtitle": "subt",
      "columnProperties": [
        {
          "header": "",
          "horizontalAlignment": "LEADING"
        },
        {
          "header": "",
          "horizontalAlignment": "LEADING"
        },
        {
          "header": "",
          "horizontalAlignment": "LEADING"
        }
      ],
      "rows": [
        {
          "cells": [
            {
              "text": "1"
            },
            {
              "text": "2"
            },
            {
              "text": "3"
            }
          ],
          "dividerAfter": false
        },
        {
          "cells": [
            {
              "text": "4"
            },
            {
              "text": "5"
            },
            {
              "text": "6"
            }
          ],
          "dividerAfter": false
        },
        {
          "cells": [
            {
              "text": "7"
            },
            {
              "text": "8"
            },
            {
              "text": "9"
            }
          ],
          "dividerAfter": false
        }
      ],
      "buttons": [
        {
          "title": "www",
          "openUrlAction": {
            "url": "https://www.idnes.cz/",
            "urlTypeHint": "URL_TYPE_HINT_UNSPECIFIED"
          }
        }
      ]
    }       
    "#;

        let messages = format!(
            r#"
      {{
        "messages": [
         {table_card}
        ]
       }}
      "#,
            table_card = table_card
        );

        println!("messages: {}", messages);

        let messages_struct: Messages = serde_json::from_str(&messages)?;
        println!("messages_struct {:#?}", messages_struct);

        let back_to_str = serde_json::to_string(&messages_struct)?;

        // this will pass...
        assert_json_eq!(
            json!(
            {
              "foo": [{
                  "header": "",
                  "horizontalAlignment": "LEADING"
                },
                {
                  "header": "",
                  "horizontalAlignment": "LEADING"
                }
              ]
            }
              ),
            json!(
              {
                "foo": [{
                    "horizontalAlignment": "LEADING",
                    "header": ""
                  },
                  {
                    "header": "",
                    "horizontalAlignment": "LEADING"
                  }
                ]
              }
            ),
        );

        // and this will fail! no idea why
        /*assert_json_eq!(
            json!(
                r#"
          {
            "foo": [{
                "header": "",
                "horizontalAlignment": "LEADING"
              },
              {
                "header": "",
                "horizontalAlignment": "LEADING"
              }
            ]
          }
          "#
            ),
            json!(
                r#"
              {
                "foo": [{
                    "horizontalAlignment": "LEADING",
                    "header": ""
                  },
                  {
                    "header": "",
                    "horizontalAlignment": "LEADING"
                  }
                ]
              }
              "#
            ),
        );*/

        // solution: using serde_json::from_str instead of providing into json! macro string literal seems
        // to produce proper serde_json::value::Value value which can be then tested properly for json structural equality by assert_json_eq

        let v1 = serde_json::from_str(
            r#"
              {
                "foo": [{
                    "header": "",
                    "horizontalAlignment": "LEADING"
                  },
                  {
                    "header": "",
                    "horizontalAlignment": "LEADING"
                  }
                ]
              }          
              "#,
        )?;

        let v2 = serde_json::from_str(
            r#"
                {
                  "foo": [{
                      "horizontalAlignment": "LEADING",
                      "header": ""
                    },
                    {
                      "header": "",
                      "horizontalAlignment": "LEADING"
                    }
                  ]
                }          
                "#,
        )?;

        println!("comapring jsons...");
        assert_json_eq!(v1, v2,);

        assert_json_eq!(
            serde_json::from_str(&messages)?,
            serde_json::from_str(&back_to_str)?
        );
        Ok(())
    }
}

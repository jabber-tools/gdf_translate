#[allow(unused_imports)]
use crate::errors::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// GDF channels we need to support:
//
// DEFAULT (Text Response + Custom Payload)
// GA (Simple Response + Basic Card + List + Suggestions Chips + Carousel Card + Browse Carousel Card + Link Out Suggestion + Media Content + Custom Payload + Table Card)
// FACEBOOK (Text Resoponse + Image + Card + Quick Replies + Custom Payload)
// SLACK (Text Resoponse + Image + Card + Quick Replies + Custom Payload) - for Twitter
// Kick / Viber (Text Resoponse + Image + Card + Quick Replies + Custom Payload) - for Whatsapp
// SKYPE (Text Resoponse + Image + Card + Quick Replies + Custom Payload)
// LINE (Text Resoponse + Image + Card + Quick Replies + Custom Payload)
// GOOGLE_HANGOUTS (Text Resoponse + Image + Card +  Custom Payload) - for wechat
//
// not supported:
//
// GDF phone gateway (Play audio, Transfer call, Synthetize speech)
// Telegram (Text Resoponse + Image + Card + Quick Replies + Custom Payload)
// RCS Business Messaging (Standalone Rich Card + Carousel Rich Card + Simple Response)

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrVecOfString {
    Str(String),
    StrArray(Vec<String>),
}

//
//
// TEXT RESPONSE
//
//
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericTextResponseType {
    #[serde(rename = "type")]
    pub message_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub speech: StringOrVecOfString,
}

//
//
// IMAGE
//
//
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericImageResponseType {
    #[serde(rename = "type")]
    pub message_type: u8,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

//
//
// QUICK REPLIES
//
//
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericQuickRepliesResponseType {
    #[serde(rename = "type")]
    pub message_type: u8,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub title: String,
    pub replies: Vec<String>,
}

//
//
// CARD
//
//
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericCardResponseButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericCardResponseType {
    #[serde(rename = "type")]
    pub message_type: u8,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub buttons: Vec<GenericCardResponseButton>,
}

//
//
// CUSTOM PAYLOADS
//
//
#[derive(Debug, Serialize, Deserialize)]
pub struct GACustomPayloadType {
    #[serde(rename = "type")]
    pub message_type: String, // platform is string
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub payload: JsonValue,
}

// platform is number (all other channels supporting custom payload)
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericCustomPayloadType {
    #[serde(rename = "type")]
    pub message_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub payload: JsonValue,
}

// no platform specified (DEFAULT CHANNEL)
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultCustomPayloadType {
    #[serde(rename = "type")]
    pub message_type: u8,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub payload: JsonValue,
}

//
//
// GOOGLE ASSISTANT (so special ;) )
//
//
#[derive(Debug, Serialize, Deserialize)]
pub struct GAImage {
    pub url: String,
    #[serde(rename = "accessibilityText")]
    pub accessibility_text: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub items: Vec<GASimpleResponseItem>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub suggestions: Vec<GASuggestionChipsTypeSuggestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GALinkOutSuggestionType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub items: Vec<GAItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GABrowseCarouselCardType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
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
    // In untagged serde enums more specific enum value must be listed before more generic values!
    // Serde will always take first match and in thus it would ignore for platform parameter
    // of facebook text response and confuse it with text response of defautl channel! We would
    // effectively loose platform param during subsequent serialization!
    GenericCustomPayload(GenericCustomPayloadType),
    GenericCardResponse(GenericCardResponseType),
    GenericImageResponse(GenericImageResponseType),
    GenericQuickRepliesResponse(GenericQuickRepliesResponseType),
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
    DefaultCustomPayload(DefaultCustomPayloadType),
    GenericTextResponse(GenericTextResponseType),
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

    /* Default channel */

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

    /* Facebook */

    #[test]
    fn test_facebook() -> Result<()> {
        let default_text_response = r#"
        {
            "type": 0,
            "lang": "en",
            "condition": "",
            "speech": "Text response"
          }
        "#;

        let facebook_text_response = r#"
        {
          "type": 0,
          "platform": "facebook",
          "lang": "en",
          "condition": "",
          "speech": "Facebook text"
        }
        "#;

        let facebook_image_response = r#"
        {
          "type": 3,
          "platform": "facebook",
          "lang": "en",
          "condition": "",
          "imageUrl": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg"
        }
        "#;

        let facebook_card_response = r#"
        {
          "type": 1,
          "platform": "facebook",
          "lang": "en",
          "condition": "",
          "title": "fb card",
          "subtitle": "subtitle",
          "imageUrl": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg",
          "buttons": [
            {
              "text": "button",
              "postback": "https://github.com/contain-rs/linked-hash-map"
            },
            {
              "text": "buitton2",
              "postback": "https://github.com/contain-rs/linked-hash-map"
            }
          ]
        }
        "#;

        let facebook_quick_replies_response = r#"
        {
          "type": 2,
          "platform": "facebook",
          "lang": "en",
          "condition": "",
          "title": "fb quick reply",
          "replies": [
            "123",
            "456"
          ]
        }
        "#;

        let facebook_custom_payload_response = r#"
        {
          "type": 2,
          "platform": "facebook",
          "lang": "en",
          "condition": "",
          "title": "fb quick reply",
          "replies": [
            "123",
            "456"
          ]
        }
        "#;

        let messages = format!(
            r#"
          {{
            "messages": [
            {default_text_response},
            {facebook_text_response},
            {facebook_image_response},
            {facebook_card_response},
            {facebook_quick_replies_response},
            {facebook_custom_payload_response}
          ]
        }}
        "#,
            default_text_response = default_text_response,
            facebook_text_response = facebook_text_response,
            facebook_image_response = facebook_image_response,
            facebook_card_response = facebook_card_response,
            facebook_quick_replies_response = facebook_quick_replies_response,
            facebook_custom_payload_response = facebook_custom_payload_response
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

    /* Skype */

    #[test]
    fn test_skype() -> Result<()> {
        let default_text_response = r#"
        {
            "type": 0,
            "lang": "en",
            "condition": "",
            "speech": "Text response"
          }
        "#;

        let skype_text_response = r#"
        {
          "type": 0,
          "platform": "skype",
          "lang": "en",
          "condition": "",
          "speech": "Skype text"
        }
        "#;

        let skype_image_response = r#"
        {
          "type": 3,
          "platform": "skype",
          "lang": "en",
          "condition": "",
          "imageUrl": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg"
        }
        "#;

        let skype_card_response = r#"
        {
          "type": 1,
          "platform": "skype",
          "lang": "en",
          "condition": "",
          "title": "card title",
          "subtitle": "skype card subtitle",
          "imageUrl": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg",
          "buttons": [
            {
              "text": "button1",
              "postback": "https://www.idnes.cz/"
            }
          ]
        }
        "#;

        let skype_quick_replies_response = r#"
        {
          "type": 2,
          "platform": "skype",
          "lang": "en",
          "condition": "",
          "title": "skype quick reply",
          "replies": [
            "yes",
            "no"
          ]
        }
        "#;

        let skype_custom_payload_response = r#"
        {
          "type": 4,
          "platform": "skype",
          "lang": "en",
          "condition": "",
          "payload": {
            "skype": {
              "text": "foo eats bar"
            }
          }
        }
        "#;

        let messages = format!(
            r#"
          {{
            "messages": [
            {default_text_response},
            {skype_text_response},
            {skype_image_response},
            {skype_card_response},
            {skype_quick_replies_response},
            {skype_custom_payload_response}
          ]
        }}
        "#,
            default_text_response = default_text_response,
            skype_text_response = skype_text_response,
            skype_image_response = skype_image_response,
            skype_card_response = skype_card_response,
            skype_quick_replies_response = skype_quick_replies_response,
            skype_custom_payload_response = skype_custom_payload_response
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

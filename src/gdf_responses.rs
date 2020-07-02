#[allow(unused_imports)]
use crate::errors::Result;
#[allow(unused_imports)]
use crate::gdf_agent::{dummy_translate, Translate};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections;

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

impl Translate for GenericTextResponseType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();
        match &self.speech {
            StringOrVecOfString::Str(str_val) => {
                map_to_translate.insert(format!("{:p}", str_val), str_val.to_owned());
            }
            StringOrVecOfString::StrArray(str_vec) => {
                for item in str_vec.iter() {
                    map_to_translate.insert(format!("{:p}", item), item.to_owned());
                }
            }
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        match &self.speech {
            StringOrVecOfString::Str(str_val) => {
                self.speech = StringOrVecOfString::Str(
                    translations_map
                        .get(&format!("{:p}", str_val))
                        .unwrap()
                        .to_owned(),
                );
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
                self.speech = StringOrVecOfString::StrArray(speech_vec);
            }
        }
    }
}

//
//
// IMAGE
//
//
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

impl Translate for GenericQuickRepliesResponseType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());

        for reply in self.replies.iter() {
            map_to_translate.insert(format!("{:p}", reply), reply.to_owned());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();

        for reply in self.replies.iter_mut() {
            *reply = translations_map
                .get(&format!("{:p}", reply))
                .unwrap()
                .to_owned();
        }
    }
}

//
//
// CARD
//
//
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
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<GenericCardResponseButton>>,
}

impl Translate for GenericCardResponseType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());
        if let Some(subtitle) = &self.subtitle {
            map_to_translate.insert(format!("{:p}", subtitle), subtitle.to_owned());
        }

        if let Some(buttons) = &self.buttons {
            for button in buttons.iter() {
                map_to_translate.extend(button.to_translation());
            }
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();

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
    }
}

//
//
// CUSTOM PAYLOADS
//
//
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GAImage {
    pub url: String,
    #[serde(rename = "accessibilityText")]
    pub accessibility_text: String,
}

impl Translate for GAImage {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(
            format!("{:p}", &self.accessibility_text),
            self.accessibility_text.to_owned(),
        );

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.accessibility_text = translations_map
            .get(&format!("{:p}", &self.accessibility_text))
            .unwrap()
            .to_owned();
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GASimpleResponseItem {
    #[serde(rename = "textToSpeech")]
    pub text_to_speech: String,
    pub ssml: String,
    #[serde(rename = "displayText")]
    pub display_text: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GASimpleResponseType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GASimpleResponseType2 {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssml: Option<String>,
    #[serde(rename = "displayText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
}

impl Translate for GASimpleResponseType2 {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        if let Some(text_to_speech) = &self.text_to_speech {
            map_to_translate.insert(format!("{:p}", text_to_speech), text_to_speech.to_owned());
        }

        if let Some(ssml) = &self.ssml {
            map_to_translate.insert(format!("{:p}", ssml), ssml.to_owned());
        }

        if let Some(display_text) = &self.display_text {
            map_to_translate.insert(format!("{:p}", display_text), display_text.to_owned());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        if let Some(text_to_speech) = &self.text_to_speech {
            self.text_to_speech = Some(
                translations_map
                    .get(&format!("{:p}", text_to_speech))
                    .unwrap()
                    .to_owned(),
            );
        }

        if let Some(ssml) = &self.ssml {
            self.ssml = Some(
                translations_map
                    .get(&format!("{:p}", ssml))
                    .unwrap()
                    .to_owned(),
            );
        }

        if let Some(display_text) = &self.display_text {
            self.display_text = Some(
                translations_map
                    .get(&format!("{:p}", display_text))
                    .unwrap()
                    .to_owned(),
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GACardTypeButton {
    pub title: String,
    #[serde(rename = "openUrlAction")]
    pub open_url_action: GAOpenUrlAction,
}

impl Translate for GACardTypeButton {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GAOpenUrlAction {
    pub url: String,
    #[serde(rename = "urlTypeHint")]
    pub url_type_hint: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GABasicCardType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "formattedText")]
    pub formatted_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<GAImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<GACardTypeButton>>,
}

impl Translate for GABasicCardType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        if let Some(title) = &self.title {
            map_to_translate.insert(format!("{:p}", title), title.to_owned());
        }

        if let Some(subtitle) = &self.subtitle {
            map_to_translate.insert(format!("{:p}", subtitle), subtitle.to_owned());
        }

        map_to_translate.insert(
            format!("{:p}", &self.formatted_text),
            self.formatted_text.to_owned(),
        );

        if let Some(image) = &self.image {
            map_to_translate.extend(image.to_translation());
        }

        if let Some(buttons) = &self.buttons {
            for button in buttons.iter() {
                map_to_translate.extend(button.to_translation())
            }
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        if let Some(title) = &self.title {
          println!(">>>>>>>>>>{}", title);
          println!(">>>>>>>>>>{}", &format!("{:p}", title));
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

        self.formatted_text = translations_map
            .get(&format!("{:p}", &self.formatted_text))
            .unwrap()
            .to_owned();

        if let Some(image) = &mut self.image {
            image.from_translation(translations_map);
        }

        if let Some(buttons) = &mut self.buttons {
            for button in buttons.iter_mut() {
                button.from_translation(translations_map);
            }
        }
    }
}

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GAItemBrowseCarousel {
    pub footer: String,
    #[serde(rename = "openUrlAction")]
    pub open_url_action: GAOpenUrlAction,
    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<GAImage>,
}

impl Translate for GAItemBrowseCarousel {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.footer), self.footer.to_owned());
        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());
        map_to_translate.insert(
            format!("{:p}", &self.description),
            self.description.to_owned(),
        );

        if let Some(image) = &self.image {
            map_to_translate.extend(image.to_translation());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.footer = translations_map
            .get(&format!("{:p}", &self.footer))
            .unwrap()
            .to_owned();

        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();

        self.description = translations_map
            .get(&format!("{:p}", &self.description))
            .unwrap()
            .to_owned();

        if let Some(image) = &mut self.image {
            image.from_translation(translations_map);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl Translate for GAListType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());
        map_to_translate.insert(format!("{:p}", &self.subtitle), self.subtitle.to_owned());

        for item in self.items.iter() {
            map_to_translate.extend(item.to_translation());
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

        for item in self.items.iter_mut() {
            item.from_translation(translations_map);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GASuggestionChipsTypeSuggestion {
    pub title: String,
}

impl Translate for GASuggestionChipsTypeSuggestion {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.title), self.title.to_owned());

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.title = translations_map
            .get(&format!("{:p}", &self.title))
            .unwrap()
            .to_owned();
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GASuggestionChipsType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub suggestions: Vec<GASuggestionChipsTypeSuggestion>,
}

impl Translate for GASuggestionChipsType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        for suggestion in self.suggestions.iter() {
            map_to_translate.extend(suggestion.to_translation());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        for suggestion in self.suggestions.iter_mut() {
            suggestion.from_translation(translations_map);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

impl Translate for GALinkOutSuggestionType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(
            format!("{:p}", &self.destination_name),
            self.destination_name.to_owned(),
        );

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.destination_name = translations_map
            .get(&format!("{:p}", &self.destination_name))
            .unwrap()
            .to_owned();
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GACarouselCardType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub items: Vec<GAItem>,
}

impl Translate for GACarouselCardType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        for item in self.items.iter() {
            map_to_translate.extend(item.to_translation());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        for item in self.items.iter_mut() {
            item.from_translation(translations_map);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GABrowseCarouselCardType {
    #[serde(rename = "type")]
    pub message_type: String,
    pub platform: String,
    pub lang: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    pub items: Vec<GAItemBrowseCarousel>,
}

impl Translate for GABrowseCarouselCardType {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        for item in self.items.iter() {
            map_to_translate.extend(item.to_translation());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        for item in self.items.iter_mut() {
            item.from_translation(translations_map);
        }
    }
}

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

pub struct NewLangMessageItem {
    pub cloned_message: MessageType,
    pub translations: collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessageType {
    // In untagged serde enums more specific enum value must be listed before more generic values!
    // Serde will always take first match and in thus it would ignore for platform parameter
    // of facebook text response and confuse it with text response of defautl channel! We would
    // effectively loose platform param during subsequent serialization!
    GenericCustomPayload(GenericCustomPayloadType),
    GenericQuickRepliesResponse(GenericQuickRepliesResponseType),
    GenericCardResponse(GenericCardResponseType),
    GenericImageResponse(GenericImageResponseType),
    GATableCard(GATableCardType),
    GACustomPayload(GACustomPayloadType),
    GABasicCard(GABasicCardType),
    GASuggestionChips(GASuggestionChipsType),
    GAList(GAListType),
    GALinkOutSuggestion(GALinkOutSuggestionType),
    GACarouselCard(GACarouselCardType),
    GABrowseCarouselCard(GABrowseCarouselCardType),
    GAMediaContent(GAMediaContentType),
    GASimpleResponse(GASimpleResponseType),
    GASimpleResponse2(GASimpleResponseType2),
    DefaultCustomPayload(DefaultCustomPayloadType),
    GenericTextResponse(GenericTextResponseType),
}

impl MessageType {
    pub fn get_message_lang(&self) -> &String {
        match self {
            MessageType::GenericCustomPayload(m) => &m.lang,
            MessageType::GenericQuickRepliesResponse(m) => &m.lang,
            MessageType::GenericCardResponse(m) => &m.lang,
            MessageType::GenericImageResponse(m) => &m.lang,
            MessageType::GATableCard(m) => &m.lang,
            MessageType::GACustomPayload(m) => &m.lang,
            MessageType::GABasicCard(m) => &m.lang,
            MessageType::GASuggestionChips(m) => &m.lang,
            MessageType::GAList(m) => &m.lang,
            MessageType::GALinkOutSuggestion(m) => &m.lang,
            MessageType::GACarouselCard(m) => &m.lang,
            MessageType::GABrowseCarouselCard(m) => &m.lang,
            MessageType::GAMediaContent(m) => &m.lang,
            MessageType::GASimpleResponse(m) => &m.lang,
            MessageType::GASimpleResponse2(m) => &m.lang,
            MessageType::DefaultCustomPayload(m) => &m.lang,
            MessageType::GenericTextResponse(m) => &m.lang,
        }
    }

    pub fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        match self {
            MessageType::GenericCustomPayload(_) => {}
            MessageType::GenericQuickRepliesResponse(m) => m.from_translation(translations_map),
            MessageType::GenericCardResponse(m) => m.from_translation(translations_map),
            MessageType::GenericImageResponse(_) => {}
            MessageType::GATableCard(m) => m.from_translation(translations_map),
            MessageType::GACustomPayload(_) => {}
            MessageType::GABasicCard(m) => m.from_translation(translations_map),
            MessageType::GASuggestionChips(m) => m.from_translation(translations_map),
            MessageType::GAList(m) => m.from_translation(translations_map),
            MessageType::GALinkOutSuggestion(m) => m.from_translation(translations_map),
            MessageType::GACarouselCard(m) => m.from_translation(translations_map),
            MessageType::GABrowseCarouselCard(m) => m.from_translation(translations_map),
            MessageType::GAMediaContent(m) => m.from_translation(translations_map),
            MessageType::GASimpleResponse(m) => m.from_translation(translations_map),
            MessageType::GASimpleResponse2(m) => m.from_translation(translations_map),
            MessageType::DefaultCustomPayload(_) => {}
            MessageType::GenericTextResponse(m) => m.from_translation(translations_map),
        }
    }

    pub fn to_new_language(&self, new_lang_code: &str) -> Option<NewLangMessageItem> {
        let mut to_translate = collections::HashMap::new();
        let cloned_message = match self {
            MessageType::GenericCustomPayload(_) => None,
            MessageType::GenericQuickRepliesResponse(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GenericQuickRepliesResponse(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::GenericCardResponse(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GenericCardResponse(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::GenericImageResponse(_) => None,
            MessageType::GATableCard(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GATableCard(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::GACustomPayload(_) => None,
            MessageType::GABasicCard(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GABasicCard(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::GASuggestionChips(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GASuggestionChips(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::GAList(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GAList(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::GALinkOutSuggestion(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GALinkOutSuggestion(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::GACarouselCard(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GACarouselCard(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::GABrowseCarouselCard(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GABrowseCarouselCard(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::GAMediaContent(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GAMediaContent(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::GASimpleResponse(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GASimpleResponse(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::GASimpleResponse2(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GASimpleResponse2(inner_msg_clone);
                Some(outer_msg_clone)
            }
            MessageType::DefaultCustomPayload(_) => None,
            MessageType::GenericTextResponse(m) => {
                let mut inner_msg_clone = m.clone();
                inner_msg_clone.lang = format!("{}", new_lang_code);
                to_translate.extend(inner_msg_clone.to_translation());
                let outer_msg_clone = MessageType::GenericTextResponse(inner_msg_clone);
                Some(outer_msg_clone)
            }
        };

        if let Some(message) = cloned_message {
            Some(NewLangMessageItem {
                cloned_message: message,
                translations: to_translate,
            })
        } else {
            None
        }
    }
}

// removes all whitespaces and replaces some characters (as produced by serde serialization)
// with entities used by DialogFlow.
pub fn normalize_json(s: &str) -> String {
    let normalized_str: String = s.split_whitespace().collect();
    normalized_str
        .replace("\n", "")
        .replace("&", "\\u0026")
        .replace("'", "\\u0027")
        .replace("<", "\\u003c")
        .replace(">", "\\u003e")
        .replace("=", "\\u003d")
}

pub fn normalize_json_for_gdf_agent_serialization(s: &str) -> String {
    let normalized_str = s
        .replace("&", "\\u0026")
        .replace("'", "\\u0027")
        .replace("<", "\\u003c")
        .replace(">", "\\u003e")
        .replace("=", "\\u003d");

    normalized_str
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translation_tests_assertions;
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

    /* Translation tests */

    // cargo test -- --show-output test_translate_generic_text_response_1
    #[test]
    fn test_translate_generic_text_response_1() -> Result<()> {
        let str_before_translation = r#"
      {
          "type": 0,
          "lang": "en",
          "condition": "",
          "speech": "Text response"
        }
      "#;

        let str_after_translation_expected = r#"
        {
          "type": 0,
          "lang": "en",
          "condition": "",
          "speech": "Text response_translated"
        }
        "#;

        translation_tests_assertions!(
            GenericTextResponseType,
            str_before_translation,
            str_after_translation_expected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_generic_text_response_2
    #[test]
    fn test_translate_generic_text_response_2() -> Result<()> {
        let str_before_translation = r#"
      {
          "type": 0,
          "lang": "en",
          "condition": "",
          "speech": ["Text response", "Text response2"]
        }
      "#;

        let str_after_translation_expected = r#"
        {
          "type": 0,
          "lang": "en",
          "condition": "",
          "speech": ["Text response_translated", "Text response2_translated"]
        }
        "#;

        translation_tests_assertions!(
            GenericTextResponseType,
            str_before_translation,
            str_after_translation_expected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_quick_reply
    #[test]
    fn test_translate_quick_reply() -> Result<()> {
        let str_before_translation = r#"
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

        let str_after_translation_expected = r#"
        {
          "type": 2,
          "platform": "facebook",
          "lang": "en",
          "condition": "",
          "title": "fb quick reply_translated",
          "replies": [
            "123_translated",
            "456_translated"
          ]
        }
        "#;

        translation_tests_assertions!(
            GenericQuickRepliesResponseType,
            str_before_translation,
            str_after_translation_expected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_generic_card
    #[test]
    fn test_translate_generic_card() -> Result<()> {
        let str_before_translation = r#"
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

        let str_after_translation_expected = r#"
        {
          "type": 1,
          "platform": "facebook",
          "lang": "en",
          "condition": "",
          "title": "fb card_translated",
          "subtitle": "subtitle_translated",
          "imageUrl": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg",
          "buttons": [
            {
              "text": "button_translated",
              "postback": "https://github.com/contain-rs/linked-hash-map"
            },
            {
              "text": "buitton2_translated",
              "postback": "https://github.com/contain-rs/linked-hash-map"
            }
          ]
        }
        "#;

        translation_tests_assertions!(
            GenericCardResponseType,
            str_before_translation,
            str_after_translation_expected
        );

        // skip optional trasnlatable fields now, i.e. now substitle, no buttons

        let str_before_translation2 = r#"
        {
          "type": 1,
          "platform": "facebook",
          "lang": "en",
          "condition": "",
          "title": "fb card",
          "imageUrl": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg"
        }
      "#;

        let str_after_translation_expected2 = r#"
        {
          "type": 1,
          "platform": "facebook",
          "lang": "en",
          "condition": "",
          "title": "fb card_translated",
          "imageUrl": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg"
        }
        "#;

        translation_tests_assertions!(
            GenericCardResponseType,
            str_before_translation2,
            str_after_translation_expected2
        );

        Ok(())
    }

    // cargo test -- --show-output test_translate_ga_simple_response
    #[test]
    fn test_translate_ga_simple_response() -> Result<()> {
        let str_before_translation = r#"
        {
          "type": "simple_response",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "items": [
            {
              "textToSpeech": "some speech",
              "ssml": "some ssml",
              "displayText": "some text"
            },
            {
              "textToSpeech": "some speech2",
              "ssml": "some ssml2",
              "displayText": "some text2"
            }
          ]
        }
      "#;

        let str_after_translation_expected = r#"
        {
          "type": "simple_response",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "items": [
            {
              "textToSpeech": "some speech_translated",
              "ssml": "some ssml_translated",
              "displayText": "some text_translated"
            },
            {
              "textToSpeech": "some speech2_translated",
              "ssml": "some ssml2_translated",
              "displayText": "some text2_translated"
            }
          ]
        }
        "#;

        translation_tests_assertions!(
            GASimpleResponseType,
            str_before_translation,
            str_after_translation_expected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_ga_simple_response2
    #[test]
    fn test_translate_ga_simple_response2() -> Result<()> {
        let str_before_translation = r#"
        {
          "type": "simple_response",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "textToSpeech": "some speech",
          "ssml": "some ssml",
          "displayText": "some text"
        }
      "#;

        let str_after_translation_expected = r#"
        {
          "type": "simple_response",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "textToSpeech": "some speech_translated",
          "ssml": "some ssml_translated",
          "displayText": "some text_translated"
        }
        "#;

        translation_tests_assertions!(
            GASimpleResponseType2,
            str_before_translation,
            str_after_translation_expected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_ga_basic_card
    #[test]
    fn test_translate_ga_basic_card() -> Result<()> {
        let str_before_translation = r#"
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

        let str_after_translation_expected = r#"
        {
          "type": "basic_card",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "title": "title_translated",
          "subtitle": "subtitle_translated",
          "formattedText": "GA simple card_translated",
          "image": {
            "url": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg",
            "accessibilityText": "accessbility text_translated"
          },
          "buttons": [
            {
              "title": "weblink title_translated",
              "openUrlAction": {
                "url": "https://github.com/contain-rs/linked-hash-map",
                "urlTypeHint": "URL_TYPE_HINT_UNSPECIFIED"
              }
            }
          ]
        }
        "#;

        translation_tests_assertions!(
            GABasicCardType,
            str_before_translation,
            str_after_translation_expected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_ga_basic_card
    #[test]
    fn test_translate_ga_browse_carousel() -> Result<()> {
        let str_before_translation = r#"
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

        let str_after_translation_expected = r#"
        {
          "type": "browse_carousel_card",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "items": [
            {
              "footer": "footer_translated",
              "openUrlAction": {
                "url": "https://www.idnes.cz/",
                "urlTypeHint": "URL_TYPE_HINT_UNSPECIFIED"
              },
              "title": "title_translated",
              "description": "desc_translated",
              "image": {
                "url": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg",
                "accessibilityText": "access text_translated"
              }
            },
            {
              "footer": "_translated",
              "openUrlAction": {
                "url": "https://www.idnes.cz/",
                "urlTypeHint": "AMP_CONTENT"
              },
              "title": "title2_translated",
              "description": "_translated",
              "image": {
                "url": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg",
                "accessibilityText": "acces text 2_translated"
              }
            }
          ]
        }     
        "#;

        translation_tests_assertions!(
            GABrowseCarouselCardType,
            str_before_translation,
            str_after_translation_expected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_ga_suggestions
    #[test]
    fn test_translate_ga_suggestions() -> Result<()> {
        let str_before_translation = r#"
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

        let str_after_translation_expected = r#"
        {
          "type": "suggestion_chips",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "suggestions": [
            {
              "title": "chip1_translated"
            },
            {
              "title": "chip2_translated"
            }
          ]
        }
        "#;

        translation_tests_assertions!(
            GASuggestionChipsType,
            str_before_translation,
            str_after_translation_expected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_ga_linkout_suggestions
    #[test]
    fn test_translate_ga_linkout_suggestions() -> Result<()> {
        let str_before_translation = r#"
        {
          "type": "link_out_chip",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "destinationName": "GA Link Out Suggestion",
          "url": "https://github.com/contain-rs/linked-hash-map"
        }  
      "#;

        let str_after_translation_expected = r#"
        {
          "type": "link_out_chip",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "destinationName": "GA Link Out Suggestion_translated",
          "url": "https://github.com/contain-rs/linked-hash-map"
        }  
        "#;

        translation_tests_assertions!(
            GALinkOutSuggestionType,
            str_before_translation,
            str_after_translation_expected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_ga_carousel_card
    #[test]
    fn test_translate_ga_carousel_card() -> Result<()> {
        let str_before_translation = r#"
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

        let str_after_translation_expected = r#"
        {
          "type": "carousel_card",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "items": [
            {
              "optionInfo": {
                "key": "key_translated",
                "synonyms": [
                  "syn_translated",
                  "sybn2_translated"
                ]
              },
              "title": "item0_translated",
              "description": "item0desc_translated",
              "image": {
                "url": "",
                "accessibilityText": "_translated"
              }
            },
            {
              "optionInfo": {
                "key": "key1_translated",
                "synonyms": [
                  "some syb_translated"
                ]
              },
              "title": "item1_translated",
              "description": "_translated",
              "image": {
                "url": "",
                "accessibilityText": "_translated"
              }
            }
          ]
        }  
        "#;

        translation_tests_assertions!(
            GACarouselCardType,
            str_before_translation,
            str_after_translation_expected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_ga_media_content
    #[test]
    fn test_translate_ga_media_content() -> Result<()> {
        let str_before_translation = r#"
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

        let str_after_translation_expected = r#"
        {
          "type": "media_content",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "mediaType": "AUDIO",
          "mediaObjects": [
            {
              "name": "cad name_translated",
              "description": "desc_translated",
              "largeImage": {
                "url": "https://i1.wp.com/www.dignited.com/wp-content/uploads/2018/09/url_istock_nicozorn_thumb800.jpg",
                "accessibilityText": "acxcess text_translated"
              },
              "contentUrl": "https://www.idnes.cz/"
            }
          ]
        }    
        "#;

        translation_tests_assertions!(
            GAMediaContentType,
            str_before_translation,
            str_after_translation_expected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_ga_table
    #[test]
    fn test_translate_ga_table() -> Result<()> {
        let str_before_translation = r#"
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

        let str_after_translation_expected = r#"
        {
          "type": "table_card",
          "platform": "google",
          "lang": "en",
          "condition": "",
          "title": "tit_translated",
          "subtitle": "subt_translated",
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
                  "text": "1_translated"
                },
                {
                  "text": "2_translated"
                },
                {
                  "text": "3_translated"
                }
              ],
              "dividerAfter": false
            },
            {
              "cells": [
                {
                  "text": "4_translated"
                },
                {
                  "text": "5_translated"
                },
                {
                  "text": "6_translated"
                }
              ],
              "dividerAfter": false
            },
            {
              "cells": [
                {
                  "text": "7_translated"
                },
                {
                  "text": "8_translated"
                },
                {
                  "text": "9_translated"
                }
              ],
              "dividerAfter": false
            }
          ],
          "buttons": [
            {
              "title": "www_translated",
              "openUrlAction": {
                "url": "https://www.idnes.cz/",
                "urlTypeHint": "URL_TYPE_HINT_UNSPECIFIED"
              }
            }
          ]
        }        
        "#;

        translation_tests_assertions!(
            GATableCardType,
            str_before_translation,
            str_after_translation_expected,
            "no_string_comparison"
        );
        Ok(())
    }
}

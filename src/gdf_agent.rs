#[allow(unused_imports)]
use crate::errors::{Error, Result};
use serde::de::{self, Deserializer};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json;

// see https://serde.rs/field-attrs.html
#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,

    #[serde(rename = "isOverridable")]
    pub is_overridable: bool,

    #[serde(rename = "isEnum")]
    pub is_enum: bool,

    #[serde(rename = "isRegexp")]
    pub is_regexp: bool,

    #[serde(rename = "automatedExpansion")]
    pub automated_expansion: bool,

    #[serde(rename = "allowFuzzyExtraction")]
    pub allow_fuzzy_extraction: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityEntry {
    pub value: String,
    pub synonyms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentManifestGoogleAssistantOauthLinking {
    pub required: bool,

    #[serde(rename = "providerId")]
    pub provider_id: String,

    #[serde(rename = "authorizationUrl")]
    pub authorization_url: String,

    #[serde(rename = "tokenUrl")]
    pub token_url: String,

    pub scopes: String,

    #[serde(rename = "privacyPolicyUrl")]
    pub privacy_policy_url: String,

    #[serde(rename = "grantType")]
    pub grant_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentManifestGoogleAssistant {
    #[serde(rename = "googleAssistantCompatible")]
    pub google_assistant_compatible: bool,

    pub project: String,

    #[serde(rename = "welcomeIntentSignInRequired")]
    pub welcome_intent_signin_required: bool,

    #[serde(rename = "startIntents")]
    pub start_intents: Vec<String>,

    #[serde(rename = "systemIntents")]
    pub system_intents: Vec<String>,

    #[serde(rename = "endIntentIds")]
    pub end_intent_ids: Vec<String>,

    #[serde(rename = "oAuthLinking")]
    pub oauth_linking: AgentManifestGoogleAssistantOauthLinking,

    #[serde(rename = "voiceType")]
    pub voice_type: String,

    pub capabilities: Vec<String>, // ??
    pub env: String,

    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,

    #[serde(rename = "autoPreviewEnabled")]
    pub auto_preview_enabled: bool,

    #[serde(rename = "isDeviceAgent")]
    pub is_device_agent: bool,
}

// TBD: there will be probably field for online code if enabled
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentManifestWebhook {
    url: String,
    username: String,
    headers: std::collections::HashMap<String, String>,
    available: bool,

    #[serde(rename = "useForDomains")]
    use_for_domains: bool,

    #[serde(rename = "cloudFunctionsEnabled")]
    cloud_functions_enabled: bool,

    #[serde(rename = "cloudFunctionsInitialized")]
    cloud_functions_initialized: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentManifest {
    pub description: String,
    pub language: String,

    #[serde(rename = "shortDescription")]
    pub short_description: String,

    pub examples: String,

    #[serde(rename = "linkToDocs")]
    pub link_to_docs: String,

    #[serde(rename = "disableInteractionLogs")]
    pub disable_interaction_logs: bool,

    #[serde(rename = "disableStackdriverLogs")]
    pub disable_stackdriver_logs: bool,

    #[serde(rename = "googleAssistant")]
    pub google_assistant: AgentManifestGoogleAssistant,

    #[serde(rename = "defaultTimezone")]
    pub default_timezone: String,

    pub webhook: AgentManifestWebhook,

    #[serde(rename = "isPrivate")]
    is_private: bool,

    #[serde(rename = "customClassifierMode")]
    pub custom_classifier_mode: String,

    #[serde(rename = "mlMinConfidence")]
    pub ml_min_confidence: f64,

    #[serde(rename = "supportedLanguages")]
    pub supported_languages: Vec<String>,

    #[serde(rename = "onePlatformApiVersion")]
    pub one_platform_api_version: String,

    #[serde(rename = "analyzeQueryTextSentiment")]
    pub analyze_query_text_sentiment: bool,

    #[serde(rename = "enabledKnowledgeBaseNames")]
    pub enabled_knowledge_base_names: Vec<String>,

    #[serde(rename = "knowledgeServiceConfidenceAdjustment")]
    pub knowledge_service_confidence_adjustment: f64,

    #[serde(rename = "dialogBuilderMode")]
    pub dialog_builder_mode: bool,

    #[serde(rename = "baseActionPackagesUrl")]
    pub base_action_packages_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentEvent {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentResponseAffectedContext {
    pub name: String,
    pub parameters: std::collections::HashMap<String, String>, // ??
    pub lifespan: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentResponseParameter {
    id: String,
    required: bool,

    #[serde(rename = "dataType")]
    data_type: String,

    name: String,
    value: String,

    #[serde(rename = "promptMessages")]
    prompt_messages: Vec<String>, // ??

    #[serde(rename = "noMatchPromptMessages")]
    no_match_prompt_messages: Vec<String>, // ??

    #[serde(rename = "noInputPromptMessages")]
    no_input_prompt_messages: Vec<String>, // ??

    #[serde(rename = "outputDialogContexts")]
    output_dialog_contexts: Vec<String>, // ??

    #[serde(rename = "isList")]
    is_list: bool,
}

#[derive(Debug)]
pub enum IntentResponseMessageType {
    TypeStr(String),
    TypeNum(i8),
}

impl Serialize for IntentResponseMessageType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            IntentResponseMessageType::TypeStr(some_str) => serializer.serialize_str(some_str),
            IntentResponseMessageType::TypeNum(some_num) => {
                serializer.serialize_i32((*some_num).into())
            } //you can convert an `i8` to `i32`: `(*some_num).into()`
        }
    }
}

fn deserialize_message_type<'de, D>(
    de: D,
) -> std::result::Result<IntentResponseMessageType, D::Error>
where
    D: Deserializer<'de>,
{
    let deser_result: serde_json::Value = Deserialize::deserialize(de)?;
    match deser_result {
        serde_json::Value::String(str_val) => Ok(IntentResponseMessageType::TypeStr(str_val)),
        serde_json::Value::Number(num_val) => {
            if let Some(i64_numval) = num_val.as_i64() {
                Ok(IntentResponseMessageType::TypeNum(i64_numval as i8))
            } else {
                Err(de::Error::custom(
                    "Invalid numeric value when deserializing IntentResponseMessage.message_type",
                ))
            }
        }
        _ => Err(de::Error::custom(
            "Unexpected value when deserializing IntentResponseMessage.message_type",
        )),
    }
}

// visitor does not work with two visit methods, no idea why, using instead: #[serde(deserialize_with="deserialize_message_type")]
/* impl<'de> Deserialize<'de> for IntentResponseMessageType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {

        struct FieldVisitor;

        //  see https://docs.serde.rs/serde/de/trait.Visitor.html
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = IntentResponseMessageType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "string or numeric value")
            }

            fn visit_str<E>(self, s: &str) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(IntentResponseMessageType::TypeStr(s.to_owned()))
            }


            fn visit_i8<E>(self, v: i8) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(IntentResponseMessageType::TypeNum(v))
            }

        }

        deserializer.deserialize_str(FieldVisitor)
    }
} */

// TBD: definitelly not covering all message types, need more love!
#[derive(Debug, Serialize, Deserialize)]
pub struct IntentResponseMessage {
    #[serde(rename = "type")]
    #[serde(deserialize_with = "deserialize_message_type")]
    pub message_type: IntentResponseMessageType, // TBD: can be string or number, see https://github.com/serde-rs/json/issues/181

    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    pub lang: String,
    pub condition: String,

    //
    // https://stackoverflow.com/questions/46993079/how-do-i-change-serdes-default-implementation-to-return-an-empty-object-instead
    // https://serde.rs/field-attrs.html
    //
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech: Option<String>,

    #[serde(rename = "textToSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssml: Option<String>,

    #[serde(rename = "displayText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentResponse {
    #[serde(rename = "resetContexts")]
    pub reset_contexts: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    #[serde(rename = "affectedContexts")]
    pub affected_contexts: Vec<IntentResponseAffectedContext>,

    pub parameters: Vec<IntentResponseParameter>,

    pub messages: Vec<IntentResponseMessage>,

    #[serde(rename = "defaultResponsePlatforms")]
    pub default_response_platforms: std::collections::HashMap<String, bool>,

    pub speech: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Intent {
    pub id: String,
    pub name: String,
    pub auto: bool,
    pub contexts: Vec<String>,

    pub responses: Vec<IntentResponse>,

    pub priority: i64,

    #[serde(rename = "webhookUsed")]
    pub webhook_used: bool,

    #[serde(rename = "webhookForSlotFilling")]
    pub webhook_for_slot_filling: bool,

    #[serde(rename = "fallbackIntent")]
    pub fallback_intent: bool,

    pub events: Vec<IntentEvent>,

    #[serde(rename = "conditionalResponses")]
    pub conditional_responses: Vec<String>, // TBD: no idea what is in these attribute, we do not use it
    pub condition: String,
    #[serde(rename = "conditionalFollowupEvents")]
    pub conditional_followup_events: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentUtteranceData {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<String>,
    #[serde(rename = "userDefined")]
    user_defined: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentUtterance {
    pub id: String,
    pub data: Vec<IntentUtteranceData>,

    #[serde(rename = "isTemplate")]
    pub is_template: bool,
    pub count: i8,
    pub updated: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use glob::glob;
    use std::fs;

    fn remove_whitespace(s: &str) -> String {
        let normalized_str: String = s.split_whitespace().collect();
        normalized_str.replace("\n", "")
    }

    #[test]
    fn test_entity_deser_ser() -> Result<()> {
        let entity_str = r#"
        {
            "id": "ed3dad98-49c6-4370-9f7e-0c6648d99820",
            "name": "additional",
            "isOverridable": true,
            "isEnum": true,
            "isRegexp": false,
            "automatedExpansion": false,
            "allowFuzzyExtraction": false
        }
        "#;
        let entity: Entity = serde_json::from_str(entity_str)?;
        assert_eq!(entity.id, "ed3dad98-49c6-4370-9f7e-0c6648d99820");
        assert_eq!(entity.name, "additional");
        assert_eq!(entity.is_overridable, true);
        assert_eq!(entity.is_enum, true);
        assert_eq!(entity.is_regexp, false);
        assert_eq!(entity.automated_expansion, false);
        assert_eq!(entity.allow_fuzzy_extraction, false);

        let serialized_str = serde_json::to_string(&entity).unwrap();
        // println!("{}",serialized_str);
        let serialized_str_expected = r#"{"id":"ed3dad98-49c6-4370-9f7e-0c6648d99820","name":"additional","isOverridable":true,"isEnum":true,"isRegexp":false,"automatedExpansion":false,"allowFuzzyExtraction":false}"#;
        assert_eq!(serialized_str, serialized_str_expected);
        Ok(())
    }

    #[test]
    fn test_entity_entries_deser_ser_1() -> Result<()> {
        let entity_entries_str = r#"
        [
            {
              "value": "additional",
              "synonyms": [
                "additional",
                "further"
              ]
            },
            {
              "value": "extra",
              "synonyms": [
                "extra"
              ]
            },
            {
              "value": "more",
              "synonyms": [
                "more"
              ]
            }
          ]
        "#;

        let entries: Vec<EntityEntry> = serde_json::from_str(entity_entries_str)?;

        assert_eq!(entries[0].value, "additional");
        assert_eq!(entries[0].synonyms.len(), 2);
        assert_eq!(entries[0].synonyms[0], "additional");
        assert_eq!(entries[0].synonyms[1], "further");

        assert_eq!(entries[1].value, "extra");
        assert_eq!(entries[1].synonyms.len(), 1);
        assert_eq!(entries[1].synonyms[0], "extra");

        assert_eq!(entries[2].value, "more");
        assert_eq!(entries[2].synonyms.len(), 1);
        assert_eq!(entries[2].synonyms[0], "more");

        let serialized_str = serde_json::to_string(&entries).unwrap();
        let serialized_str_expected = r#"[{"value":"additional","synonyms":["additional","further"]},{"value":"extra","synonyms":["extra"]},{"value":"more","synonyms":["more"]}]"#;
        assert_eq!(serialized_str, serialized_str_expected);

        Ok(())
    }

    #[test]
    fn test_entity_entries_deser_ser_2() -> Result<()> {
        let entity_entries_str = r#"
        [
            {
              "value": "(?i)j?jd[01]{2}\\d{14,16}",
              "synonyms": []
            }
        ]
        "#;

        let entries: Vec<EntityEntry> = serde_json::from_str(entity_entries_str)?;

        assert_eq!(entries.len(), 1);

        assert_eq!(entries[0].value, "(?i)j?jd[01]{2}\\d{14,16}");
        assert_eq!(entries[0].synonyms.len(), 0);

        let serialized_str = serde_json::to_string(&entries).unwrap();
        let serialized_str_expected = r#"[{"value":"(?i)j?jd[01]{2}\\d{14,16}","synonyms":[]}]"#;
        assert_eq!(serialized_str, serialized_str_expected);

        Ok(())
    }

    #[test]
    fn test_package_deser_ser() -> Result<()> {
        let package_str = r#"
        {
            "version": "1.0.0"
        }
        "#;
        let package: Package = serde_json::from_str(package_str)?;
        assert_eq!(package.version, "1.0.0");

        let serialized_str = serde_json::to_string(&package).unwrap();
        let serialized_str_expected = r#"{"version":"1.0.0"}"#;
        assert_eq!(
            remove_whitespace(&serialized_str),
            remove_whitespace(serialized_str_expected)
        );
        Ok(())
    }

    #[test]
    fn test_agent_manifest_deser_ser() -> Result<()> {
        let agent_str = r#"
        {
            "description": "Express tracking bot",
            "language": "en",
            "shortDescription": "",
            "examples": "",
            "linkToDocs": "",
            "disableInteractionLogs": false,
            "disableStackdriverLogs": false,
            "googleAssistant": {
              "googleAssistantCompatible": true,
              "project": "express-tracking",
              "welcomeIntentSignInRequired": true,
              "startIntents": [],
              "systemIntents": [],
              "endIntentIds": [],
              "oAuthLinking": {
                "required": false,
                "providerId": "",
                "authorizationUrl": "",
                "tokenUrl": "",
                "scopes": "",
                "privacyPolicyUrl": "",
                "grantType": "AUTH_CODE_GRANT"
              },
              "voiceType": "MALE_1",
              "capabilities": [],
              "env": "",
              "protocolVersion": "V2",
              "autoPreviewEnabled": false,
              "isDeviceAgent": false
            },
            "defaultTimezone": "Europe/Madrid",
            "webhook": {
              "url": "https://express-cs-enterprise.appspot.com/fulfilment",
              "username": "",
              "headers": {
                "": ""
              },
              "available": true,
              "useForDomains": false,
              "cloudFunctionsEnabled": false,
              "cloudFunctionsInitialized": false
            },
            "isPrivate": true,
            "customClassifierMode": "use.after",
            "mlMinConfidence": 0.5,
            "supportedLanguages": [
              "es",
              "pt-br"
            ],
            "onePlatformApiVersion": "v2",
            "analyzeQueryTextSentiment": true,
            "enabledKnowledgeBaseNames": [],
            "knowledgeServiceConfidenceAdjustment": -0.4,
            "dialogBuilderMode": false,
            "baseActionPackagesUrl": ""
          }
        "#;
        let agent: AgentManifest = serde_json::from_str(agent_str)?;
        assert_eq!(agent.description, "Express tracking bot");
        assert_eq!(agent.knowledge_service_confidence_adjustment, -0.4);
        assert_eq!(
            agent.google_assistant.oauth_linking.grant_type,
            "AUTH_CODE_GRANT"
        );

        let serialized_str = serde_json::to_string(&agent).unwrap();
        assert_eq!(
            remove_whitespace(&serialized_str),
            remove_whitespace(agent_str)
        );
        Ok(())
    }

    #[test]
    fn test_intent_deser_ser_1() -> Result<()> {
        let intent_str = r#"
        {
          "id": "d9d7d680-8adc-4571-b2bf-22ba3c5dbc75",
          "name": "FAQ|CS|0|Stop ODD Messages|TPh",
          "auto": true,
          "contexts": [],
          "responses": [
            {
              "resetContexts": false,
              "action": "country_specific_response",
              "affectedContexts": [],
              "parameters": [
                {
                  "id": "d3f16abc-1032-4e1a-a3ea-fa520f0d1b4f",
                  "required": false,
                  "dataType": "",
                  "name": "countries",
                  "value": "CA",
                  "promptMessages": [],
                  "noMatchPromptMessages": [],
                  "noInputPromptMessages": [],
                  "outputDialogContexts": [],
                  "isList": false
                },
                {
                  "id": "6257c129-4817-488d-83ee-57b72632b86b",
                  "required": false,
                  "dataType": "",
                  "name": "event",
                  "value": "faq_stop_odd",
                  "promptMessages": [],
                  "noMatchPromptMessages": [],
                  "noInputPromptMessages": [],
                  "outputDialogContexts": [],
                  "isList": false
                },
                {
                  "id": "3274c173-4243-42fa-bdbe-7ced43f64d53",
                  "required": false,
                  "dataType": "@no",
                  "name": "no",
                  "value": "$no",
                  "promptMessages": [],
                  "noMatchPromptMessages": [],
                  "noInputPromptMessages": [],
                  "outputDialogContexts": [],
                  "isList": false
                }
              ],
              "messages": [
                {
                  "type": "simple_response",
                  "platform": "google",
                  "lang": "pt-br",
                  "condition": "",
                  "textToSpeech": "PLACEHOLDER - NÃ£o altere esta cÃ©lula",
                  "ssml": "",
                  "displayText": ""
                }
              ],
              "defaultResponsePlatforms": {},
              "speech": ["xixix"]
            }
          ],
          "priority": 500000,
          "webhookUsed": false,
          "webhookForSlotFilling": false,
          "fallbackIntent": false,
          "events": [
            {
              "name": "faq_stop_odd_entry"
            }
          ],
          "conditionalResponses": [],
          "condition": "",
          "conditionalFollowupEvents": []
        }
        "#;
        let intent: Intent = serde_json::from_str(intent_str)?;
        assert_eq!(intent.name, "FAQ|CS|0|Stop ODD Messages|TPh");

        let serialized_str = serde_json::to_string(&intent).unwrap();
        assert_eq!(
            remove_whitespace(&serialized_str),
            remove_whitespace(&intent_str)
        );
        Ok(())
    }

    #[test]
    fn test_intent_deser_ser_2() -> Result<()> {
        let intent_str = r#"
        {
          "id": "d9d7d680-8adc-4571-b2bf-22ba3c5dbc75",
          "name": "FAQ|CS|0|Stop ODD Messages|TPh",
          "auto": true,
          "contexts": [],
          "responses": [
            {
              "resetContexts": false,
              "action": "country_specific_response",
              "affectedContexts": [],
              "parameters": [
                {
                  "id": "d3f16abc-1032-4e1a-a3ea-fa520f0d1b4f",
                  "required": false,
                  "dataType": "",
                  "name": "countries",
                  "value": "CA",
                  "promptMessages": [],
                  "noMatchPromptMessages": [],
                  "noInputPromptMessages": [],
                  "outputDialogContexts": [],
                  "isList": false
                },
                {
                  "id": "6257c129-4817-488d-83ee-57b72632b86b",
                  "required": false,
                  "dataType": "",
                  "name": "event",
                  "value": "faq_stop_odd",
                  "promptMessages": [],
                  "noMatchPromptMessages": [],
                  "noInputPromptMessages": [],
                  "outputDialogContexts": [],
                  "isList": false
                },
                {
                  "id": "3274c173-4243-42fa-bdbe-7ced43f64d53",
                  "required": false,
                  "dataType": "@no",
                  "name": "no",
                  "value": "$no",
                  "promptMessages": [],
                  "noMatchPromptMessages": [],
                  "noInputPromptMessages": [],
                  "outputDialogContexts": [],
                  "isList": false
                }
              ],
              "messages": [
                {
                    "type": 0,
                    "lang": "en",
                    "condition": "",
                    "speech": "You can contact our Technical Support team on 1-855-123-4567"
                  },
                  {
                    "type": "simple_response",
                    "platform": "google",
                    "lang": "en",
                    "condition": "",
                    "textToSpeech": "You can contact our Technical Support team on 1-855-123-4567",
                    "ssml": "",
                    "displayText": ""
                  }
              ],
              "defaultResponsePlatforms": {},
              "speech": ["xixix"]
            }
          ],
          "priority": 500000,
          "webhookUsed": false,
          "webhookForSlotFilling": false,
          "fallbackIntent": false,
          "events": [
            {
              "name": "faq_stop_odd_entry"
            }
          ],
          "conditionalResponses": [],
          "condition": "",
          "conditionalFollowupEvents": []
        }
        "#;
        let intent: Intent = serde_json::from_str(intent_str)?;
        assert_eq!(intent.name, "FAQ|CS|0|Stop ODD Messages|TPh");

        let serialized_str = serde_json::to_string(&intent).unwrap();
        assert_eq!(
            remove_whitespace(&serialized_str),
            remove_whitespace(&intent_str)
        );
        Ok(())
    }

    #[test]
    fn test_intent_utterance_deser_ser_1() -> Result<()> {
        let intent_utterance_str = r#"
        [
          {
            "id": "9dfa147d-d2d8-4703-a693-8edef11322a2",
            "data": [
              {
                "text": "FAQ|BIT|0|Tech Support|CA",
                "userDefined": false
              }
            ],
            "isTemplate": true,
            "count": 123,
            "updated": 456
          }
        ]       
        "#;
        let intent_utterances: Vec<IntentUtterance> = serde_json::from_str(intent_utterance_str)?;
        assert_eq!(
            intent_utterances[0].id,
            "9dfa147d-d2d8-4703-a693-8edef11322a2"
        );
        assert_eq!(intent_utterances[0].data.len(), 1);
        assert_eq!(
            intent_utterances[0].data[0].text,
            "FAQ|BIT|0|Tech Support|CA"
        );
        assert_eq!(intent_utterances[0].data[0].user_defined, false);
        assert_eq!(intent_utterances[0].is_template, true);
        assert_eq!(intent_utterances[0].count, 123);
        assert_eq!(intent_utterances[0].updated, 456);

        let serialized_str = serde_json::to_string(&intent_utterances).unwrap();
        assert_eq!(
            remove_whitespace(&serialized_str),
            remove_whitespace(&intent_utterance_str)
        );
        Ok(())
    }

    #[test]
    fn test_entity_entries() -> Result<()> {
        for entry in glob("./examples/testdata/entities/*_entries_*.json")
            .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => {
                    let file_name = path.as_path().to_str().unwrap();
                    // println!("processing file {}", file_name);
                    let file_str = fs::read_to_string(file_name)?;

                    let deserialized_struct: Vec<EntityEntry> = serde_json::from_str(&file_str)?;

                    let serialized_str = serde_json::to_string(&deserialized_struct).unwrap();
                    assert_eq!(
                        remove_whitespace(&serialized_str)
                            .replace("&", "\\u0026") // these two discrepancies found in Express_CS_AM_PRD !
                            .replace("'", "\\u0027"),
                        remove_whitespace(&file_str)
                    );
                }
                Err(e) => {
                    println!("error when processing file");
                    panic!(e);
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_entities() -> Result<()> {
        for entry in
            glob("./examples/testdata/entities/*.json").expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => {
                    let file_name = path.as_path().to_str().unwrap();
                    if file_name.contains("_entries_") {
                        continue; // skip entries, process entities only!
                    }
                    // println!("processing file {}", file_name);
                    let file_str = fs::read_to_string(file_name)?;

                    let deserialized_struct: Entity = serde_json::from_str(&file_str)?;

                    let serialized_str = serde_json::to_string(&deserialized_struct).unwrap();
                    assert_eq!(
                        remove_whitespace(&serialized_str),
                        remove_whitespace(&file_str)
                    );
                }
                Err(e) => {
                    println!("error when processing file");
                    panic!(e);
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_utterances() -> Result<()> {
        for entry in glob("./examples/testdata/intents/*_usersays_*.json")
            .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => {
                    let file_name = path.as_path().to_str().unwrap();
                    // println!("processing file {}", file_name);
                    let file_str = fs::read_to_string(file_name)?;

                    let deserialized_struct: Vec<IntentUtterance> =
                        serde_json::from_str(&file_str)?;

                    let serialized_str = serde_json::to_string(&deserialized_struct).unwrap();
                    assert_eq!(
                        remove_whitespace(&serialized_str).replace("'", "\\u0027"),
                        remove_whitespace(&file_str)
                    );
                }
                Err(e) => {
                    println!("error when processing file");
                    panic!(e);
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_intents() -> Result<()> {
        for entry in
            glob("./examples/testdata/intents/*.json").expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => {
                    let file_name = path.as_path().to_str().unwrap();
                    if file_name.contains("_usersays_") {
                        continue; // skip utterances, process intents only!
                    }
                    println!("processing file {}", file_name);
                    let file_str = fs::read_to_string(file_name)?;

                    let deserialized_struct: Intent = serde_json::from_str(&file_str)?;

                    let serialized_str = serde_json::to_string(&deserialized_struct).unwrap();
                    assert_eq!(
                        remove_whitespace(&serialized_str).replace("<", "\\u003c").replace(">", "\\u003e").replace("=", "\\u003d"),
                        remove_whitespace(&file_str)
                    );
                }
                Err(e) => {
                    println!("error when processing file");
                    panic!(e);
                }
            }
        }

        Ok(())
    }
}

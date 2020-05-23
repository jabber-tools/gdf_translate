#[allow(unused_imports)]
use crate::errors::Result;
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

#[cfg(test)]
mod tests {
    use super::*;

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
}

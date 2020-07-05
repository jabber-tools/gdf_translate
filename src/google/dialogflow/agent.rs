mod entities;
mod intents;
mod utterances;

use entities::*;
use intents::*;
use utterances::*;

use crate::errors::{Error, Result};
use crate::google::dialogflow::responses::normalize_json_for_gdf_agent_serialization;
use crate::parse_gdf_agent_files;
use crate::serialize_gdf_agent_section;
use crate::zip::{unzip_file, zip_directory};
use assert_json_diff::assert_json_eq_no_panic;
use glob::glob;
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections;
use std::env::current_exe;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub trait Translate {
    fn to_translation(&self) -> collections::HashMap<String, String>;
    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>);
}

lazy_static! {
    pub static ref RE_ENTITY_ENTRY_FILE: Regex =
        Regex::new(r"(\w+entries_)([a-zA-Z-]+).json").unwrap();
    pub static ref RE_INTENT_UTTERANCE_FILE: Regex =
        Regex::new(r"(\w+usersays_)([a-zA-Z-]+).json").unwrap();
    pub static ref RE_COMPOSITE_ENTITY: Regex = Regex::new(r"@\w+:\w+").unwrap();
}

// used in unit tests in gdf_responses and gdf_agent
pub fn dummy_translate(translation_map: &mut collections::HashMap<String, String>) {
    for val in translation_map.values_mut() {
        let translated_text = format!("{}{}", val, "_translated");
        *val = translated_text;
    }
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

    pub capabilities: Vec<String>,
    pub env: String,

    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,

    #[serde(rename = "autoPreviewEnabled")]
    pub auto_preview_enabled: bool,

    #[serde(rename = "isDeviceAgent")]
    pub is_device_agent: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentManifestWebhook {
    url: String,
    username: String,
    headers: collections::HashMap<String, String>,
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

#[derive(Debug)]
pub struct GoogleDialogflowAgent {
    entities: Vec<EntityFile>,
    entity_entries: Vec<EntityEntriesFile>,
    intents: Vec<IntentFile>,
    utterances: Vec<IntentUtterancesFile>,
    agent: AgentManifest,
    package: Package,
}

impl GoogleDialogflowAgent {
    fn new(
        entities: Vec<EntityFile>,
        entity_entries: Vec<EntityEntriesFile>,
        intents: Vec<IntentFile>,
        utterances: Vec<IntentUtterancesFile>,
        agent: AgentManifest,
        package: Package,
    ) -> Self {
        GoogleDialogflowAgent {
            entities,
            entity_entries,
            intents,
            utterances,
            agent,
            package,
        }
    }

    // entity entries file is something like sys.color_entries_en.json
    // we need to calculate lenght of '_entries' + 'en' so that we can remove
    // it and get entity master file name, i.e. sys.color.json
    fn entity_entry_file_name_to_entity_filename(entity_entry_file_name: &str) -> String {
        let caps = RE_ENTITY_ENTRY_FILE
            .captures(entity_entry_file_name)
            .unwrap();

        let suffix_len = &caps[2].len() + 14; // 14 = len(_entries_)  + len (.json). suffix len = 14 + len(lang code)
        let prefix_len = entity_entry_file_name.len() - suffix_len;
        let entity_file_name = &entity_entry_file_name[0..prefix_len];
        format!("{}{}", entity_file_name, ".json")
    }

    pub fn to_translation(
        &mut self,
        lang_from: &str,
        lang_to: &str,
    ) -> collections::HashMap<String, String> {
        let mut translations_map: collections::HashMap<String, String> =
            collections::HashMap::new();

        // create new entity entry files and add their content to map to translate
        let mut new_entity_entry_files = vec![];
        for entity_entry_file in self.entity_entries.iter() {
            let caps = RE_ENTITY_ENTRY_FILE
                .captures(&entity_entry_file.file_name)
                .unwrap();

            let entity_file_name = GoogleDialogflowAgent::entity_entry_file_name_to_entity_filename(
                &entity_entry_file.file_name,
            );

            let entity_files: Vec<EntityFile> = self
                .entities
                .iter()
                .filter(|entity| entity.file_name == entity_file_name)
                .cloned()
                .collect();
            if entity_files[0].file_content.is_regexp == true {
                // we are skipping regex entities
                continue;
            }

            if &caps[2] == lang_from {
                new_entity_entry_files.push(entity_entry_file.to_new_language(lang_to));
            }
        }

        for new_entity_entry_file in new_entity_entry_files.iter() {
            for new_entity_entry in new_entity_entry_file.file_content.iter() {
                if RE_COMPOSITE_ENTITY.is_match(&new_entity_entry.value) == false
                /* skip composite entities*/
                {
                    translations_map.extend(new_entity_entry.to_translation());
                }
            }
        }

        self.entity_entries.extend(new_entity_entry_files);

        // create new intent utterance files and add their content to map to translate
        let mut new_utterance_files = vec![];
        for utterance_file in self.utterances.iter() {
            let caps = RE_INTENT_UTTERANCE_FILE
                .captures(&utterance_file.file_name)
                .unwrap();

            if &caps[2] == lang_from {
                new_utterance_files.push(utterance_file.to_new_language(lang_to));
            }
        }

        for new_utterance_file in new_utterance_files.iter() {
            for utterance in new_utterance_file.file_content.iter() {
                for utterance_data in utterance.data.iter() {
                    translations_map.extend(utterance_data.to_translation());
                }
            }
        }

        self.utterances.extend(new_utterance_files);

        // first find intents that should not be translated
        let mut intents_not_to_translate = vec![];
        'intent_loop: for intent_file in self.intents.iter() {
            let intent = &intent_file.file_content;
            for intent_response in intent.responses.iter() {
                for intent_response_message in intent_response.messages.iter() {
                    if intent_response_message.get_message_lang() == lang_to {
                        // if intent has already messages in target language just skip it
                        // DialogFlow will translate some intent sby default when new lang is added
                        // e.g. Default Welcome Intent, Fallback
                        intents_not_to_translate.push(intent.name.to_string());
                        continue 'intent_loop;
                    }
                }
            }
        }

        // now iterate intent file again this time already skipping the intents
        // which are already translated...
        for intent_file in self.intents.iter_mut() {
            let intent = &mut intent_file.file_content;
            if intents_not_to_translate.contains(&intent.name) {
                continue;
            }

            //... for those taht still needs to be translated iterate all responses in source language
            // clone them (while changing the target language) + add the references' addresses into translation map
            for intent_response in intent.responses.iter_mut() {
                let mut new_messages = vec![];
                for intent_response_message in intent_response.messages.iter() {
                    if intent_response_message.get_message_lang() == lang_from {
                        let new_message = intent_response_message.new_message(lang_to);
                        if let Some(message) = new_message {
                            new_messages.push(message);
                        }
                    }
                }
                intent_response.messages.extend(new_messages);
            }
        }

        for intent_file in self.intents.iter() {
            for intent_response in intent_file.file_content.responses.iter() {
                for message in intent_response.messages.iter() {
                    if message.get_message_lang() == lang_to {
                        translations_map.extend(message.to_translation());
                    }
                }
            }
        }

        translations_map
    }

    pub fn from_translation(
        &mut self,
        translations_map: &collections::HashMap<String, String>,
        lang_to: &str,
    ) {
        for entity_entry_file in self.entity_entries.iter_mut() {
            for entity_entry in entity_entry_file.file_content.iter_mut() {
                entity_entry.from_translation(translations_map);
            }
        }

        for utterances_file in self.utterances.iter_mut() {
            for utterance_file in utterances_file.file_content.iter_mut() {
                for utterance_data in utterance_file.data.iter_mut() {
                    utterance_data.from_translation(translations_map);
                }
            }
        }

        for intent_file in self.intents.iter_mut() {
            for intent_response in intent_file.file_content.responses.iter_mut() {
                for message in intent_response.messages.iter_mut() {
                    if message.get_message_lang() == lang_to {
                        message.from_translation(translations_map);
                    }
                }
            }
        }
    }

    pub fn serialize(&self, target_folder: &str) -> Result<()> {
        let base_path = Path::new(target_folder);
        let unpacked_folder = base_path.join("_unpacked");
        let intents_folder = unpacked_folder.join("intents");
        let entities_folder = unpacked_folder.join("entities");
        let packed_folder = base_path.join("_packed");

        fs::create_dir_all(&intents_folder)?;
        fs::create_dir_all(&entities_folder)?;
        fs::create_dir_all(&packed_folder)?;

        let package_file_str = serde_json::to_string_pretty(&self.package)?;
        let mut package_file_handle = File::create(unpacked_folder.join("package.json"))?;
        package_file_handle.write_all(package_file_str.as_bytes())?;

        let agent_file_str = serde_json::to_string_pretty(&self.agent)?;
        let mut agent_file_handle = File::create(unpacked_folder.join("agent.json"))?;
        agent_file_handle.write_all(agent_file_str.as_bytes())?;

        serialize_gdf_agent_section!(self.entities.iter(), entities_folder);
        serialize_gdf_agent_section!(self.entity_entries.iter(), entities_folder);
        serialize_gdf_agent_section!(self.intents.iter(), intents_folder);
        serialize_gdf_agent_section!(self.utterances.iter(), intents_folder);

        zip_directory(
            unpacked_folder.to_str().unwrap(),
            packed_folder.join("TranslatedAgent.zip").to_str().unwrap(),
        )?;
        Ok(())
    }
} // impl GoogleDialogflowAgent

parse_gdf_agent_files!(parse_gdf_agent_files_entity, Entity, EntityFile);
parse_gdf_agent_files!(
    parse_gdf_agent_files_entity_entries,
    Vec<EntityEntry>,
    EntityEntriesFile
);
parse_gdf_agent_files!(parse_gdf_agent_files_intent, Intent, IntentFile);
parse_gdf_agent_files!(
    parse_gdf_agent_files_intent_utterances,
    Vec<IntentUtterance>,
    IntentUtterancesFile
);

#[allow(dead_code)]
fn parse_gdf_agent_zip(zip_path: &str) -> Result<GoogleDialogflowAgent> {
    // create temp folder name as epoch time in sec
    let ts_sec = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // from curent binary path remove executable name (e.g. ddf_translate.exe) and add temp folder name
    let tmp_working_folder_path = current_exe()?
        .into_boxed_path()
        .as_ref()
        .parent()
        .unwrap()
        .join(Path::new(&ts_sec.to_string()));

    let agent_manifest_file = tmp_working_folder_path.join("agent.json");
    let package_file = tmp_working_folder_path.join("package.json");

    // create glob search expression <<tmp_working_folder_path>>/entities/*_entries_*.json
    let mut glob_entity_entries = tmp_working_folder_path.clone();
    glob_entity_entries.push("entities");
    glob_entity_entries.push("*_entries_*.json");
    debug!("glob_entity_entries={:?}", glob_entity_entries);

    // create glob search expression <<tmp_working_folder_path>>/entities/*_entries_*.json
    let mut glob_entities = tmp_working_folder_path.clone();
    glob_entities.push("entities");
    glob_entities.push("*.json");
    debug!("glob_entities={:?}", glob_entities);

    // create glob search expression <<tmp_working_folder_path>>/intents/*_usersays_*.json
    let mut glob_intents_usersays = tmp_working_folder_path.clone();
    glob_intents_usersays.push("intents");
    glob_intents_usersays.push("*_usersays_*.json");
    debug!("glob_intents_usersays={:?}", glob_intents_usersays);

    // create glob search expression <<tmp_working_folder_path>>/intents/*.json
    let mut glob_intents = tmp_working_folder_path.clone();
    glob_intents.push("intents");
    glob_intents.push("*.json");
    debug!("glob_intents={:?}", glob_intents);

    // convert to string slice
    let tmp_working_folder_path = tmp_working_folder_path.to_str().unwrap();

    println!("creating folder={}", tmp_working_folder_path);
    fs::create_dir_all(tmp_working_folder_path)?;
    unzip_file(zip_path, tmp_working_folder_path)?;

    let entities = parse_gdf_agent_files_entity(&glob_entities)?;
    let entity_entries = parse_gdf_agent_files_entity_entries(&glob_entity_entries)?;
    let intents = parse_gdf_agent_files_intent(&glob_intents)?;
    let utterances = parse_gdf_agent_files_intent_utterances(&glob_intents_usersays)?;

    // process agent.json
    let file_str = fs::read_to_string(agent_manifest_file)?;
    let agent_manifest: AgentManifest = serde_json::from_str(&file_str)?;
    let serialized_str = serde_json::to_string(&agent_manifest).unwrap();
    let comparison_result = assert_json_eq_no_panic(
        &serde_json::from_str(&serialized_str)?,
        &serde_json::from_str(&file_str)?,
    );

    if let Err(err_msg) = comparison_result {
        return Err(Error::new(err_msg));
    }

    // process package.json
    let file_str = fs::read_to_string(package_file)?;
    let package: Package = serde_json::from_str(&file_str)?;
    let serialized_str = serde_json::to_string(&package).unwrap();
    let comparison_result = assert_json_eq_no_panic(
        &serde_json::from_str(&serialized_str)?,
        &serde_json::from_str(&file_str)?,
    );

    if let Err(err_msg) = comparison_result {
        return Err(Error::new(err_msg));
    }

    Ok(GoogleDialogflowAgent::new(
        entities,
        entity_entries,
        intents,
        utterances,
        agent_manifest,
        package,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::google::dialogflow::responses::normalize_json;
    use crate::translation_tests_assertions;
    use assert_json_diff::assert_json_eq;

    const SAMPLE_AGENTS_FOLDER: &str =
        "C:/Users/abezecny/adam/WORK/_DEV/Rust/gdf_translate/examples/sample_agents/";

    #[derive(Debug)]
    struct DummyStructSlave {
        pub foo: String,
        pub bar: String,
        pub id: String,
    }

    impl DummyStructSlave {
        fn new(foo: String, bar: String, id: String) -> Self {
            DummyStructSlave { foo, bar, id }
        }
    }

    impl Translate for DummyStructSlave {
        fn to_translation(&self) -> collections::HashMap<String, String> {
            let mut map_to_translate = collections::HashMap::new();

            let ptr_addr_foo = format!("{:p}", &self.foo);
            let ptr_addr_bar = format!("{:p}", &self.bar);
            map_to_translate.insert(ptr_addr_foo, self.foo.to_owned());
            map_to_translate.insert(ptr_addr_bar, self.bar.to_owned());

            map_to_translate
        }

        fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
            let ptr_addr_foo = format!("{:p}", &self.foo);
            let ptr_addr_bar = format!("{:p}", &self.bar);
            let translated_foo = translations_map.get(&ptr_addr_foo).unwrap();
            let translated_bar = translations_map.get(&ptr_addr_bar).unwrap();
            self.foo = translated_foo.to_owned();
            self.bar = translated_bar.to_owned();
        }
    }

    #[derive(Debug)]
    struct DummyStructMaster {
        pub items: Vec<DummyStructSlave>,
    }

    impl DummyStructMaster {
        fn new(items: Vec<DummyStructSlave>) -> Self {
            DummyStructMaster { items }
        }
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
            normalize_json(&serialized_str),
            normalize_json(serialized_str_expected)
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
        assert_eq!(normalize_json(&serialized_str), normalize_json(agent_str));
        Ok(())
    }

    #[test]
    // no speech specified for response message
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
              "speech": []
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
        assert_eq!(normalize_json(&serialized_str), normalize_json(&intent_str));
        Ok(())
    }

    #[test]
    // speech with single string specified  + array of responses for response message
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
                    "type": 0,
                    "lang": "en",
                    "condition": "",
                    "speech": ["You can contact our Technical Support team on 1-855-123-4567", "second response here"]
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
              "speech": []
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
        assert_eq!(normalize_json(&serialized_str), normalize_json(&intent_str));
        Ok(())
    }

    #[test]
    // intent with parameter prompts
    fn test_intent_deser_ser_3() -> Result<()> {
        let intent_str = r#"
        {
            "id": "85ee820c-b534-457e-a943-eb89570e074b",
            "name": "bot.order",
            "auto": true,
            "contexts": [],
            "responses": [
              {
                "resetContexts": false,
                "action": "bot.order",
                "affectedContexts": [
                  {
                    "name": "botorder-followup",
                    "parameters": {},
                    "lifespan": 2
                  }
                ],
                "parameters": [
                  {
                    "id": "a666588f-0e23-44a0-bafb-c39ee0067706",
                    "required": true,
                    "dataType": "@industry",
                    "name": "industry",
                    "value": "$industry",
                    "prompts": [
                      {
                        "lang": "en",
                        "value": "What\u0027s the industry you\u0027re working at?"
                      },
                      {
                        "lang": "en",
                        "value": "What field are you working in?"
                      }
                    ],
                    "promptMessages": [],
                    "noMatchPromptMessages": [],
                    "noInputPromptMessages": [],
                    "outputDialogContexts": [],
                    "isList": false
                  },
                  {
                    "id": "017d987e-f30f-4f01-b946-3329d10b910b",
                    "required": true,
                    "dataType": "@platform",
                    "name": "platform",
                    "value": "$platform",
                    "prompts": [
                      {
                        "lang": "en",
                        "value": "What platform are you launching for?"
                      }
                    ],
                    "promptMessages": [],
                    "noMatchPromptMessages": [],
                    "noInputPromptMessages": [],
                    "outputDialogContexts": [],
                    "isList": false
                  }
                ],
                "messages": [
                  {
                    "type": 2,
                    "platform": "slack",
                    "lang": "en",
                    "condition": "",
                    "title": "",
                    "replies": [
                      "Yes",
                      "No"
                    ]
                  },
                  {
                    "type": 2,
                    "platform": "facebook",
                    "lang": "en",
                    "condition": "",
                    "title": "",
                    "replies": [
                      "Yes",
                      "No"
                    ]
                  },
                  {
                    "type": "suggestion_chips",
                    "platform": "google",
                    "lang": "en",
                    "condition": "",
                    "suggestions": [
                      {
                        "title": "Yes"
                      },
                      {
                        "title": "No"
                      }
                    ]
                  },
                  {
                    "type": 0,
                    "lang": "en",
                    "condition": "",
                    "speech": "Are you planning to use a web service?"
                  }
                ],
                "defaultResponsePlatforms": {
                    "facebook": true,
                    "slack": true,
                    "google": true
                },
                "speech": []
              }
            ],
            "priority": 500000,
            "webhookUsed": false,
            "webhookForSlotFilling": false,
            "fallbackIntent": false,
            "events": [],
            "conditionalResponses": [],
            "condition": "",
            "conditionalFollowupEvents": []
          }
        "#;
        let intent: Intent = serde_json::from_str(intent_str)?;
        assert_eq!(intent.name, "bot.order");

        let serialized_str = serde_json::to_string(&intent).unwrap();
        // cannot compare strings due to defaultResponsePlatforms which can come out in different order
        // assert_eq!(normalize_json(&serialized_str), normalize_json(&intent_str));

        let comparison_result = assert_json_eq_no_panic(
            &serde_json::from_str(&serialized_str)?,
            &serde_json::from_str(&intent_str)?,
        );

        if let Err(err_msg) = comparison_result {
            return Err(Error::new(err_msg));
        }

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
            normalize_json(&serialized_str),
            normalize_json(&intent_utterance_str)
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
                    assert_eq!(normalize_json(&serialized_str), normalize_json(&file_str));
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
                    assert_eq!(normalize_json(&serialized_str), normalize_json(&file_str));
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
                    assert_eq!(normalize_json(&serialized_str), normalize_json(&file_str));
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
                    let serialized_str = serde_json::to_string(&deserialized_struct)?;

                    println!("deserialized_struct: {:#?}", deserialized_struct);
                    println!("serialized_str: {}", serialized_str);
                    assert_json_eq!(
                        serde_json::from_str(&serialized_str)?,
                        serde_json::from_str(&file_str)?
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

    // cargo test -- --show-output test_file_regex_operations
    #[test]
    fn test_file_regex_operations() {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\w+entries_([a-zA-Z-]+).json").unwrap();
        }

        let file_name = "express_country_entries_en.json";
        let caps = RE.captures(file_name).unwrap();
        let lang_code = caps.get(1).unwrap().as_str();
        assert_eq!(lang_code, "en");

        let file_name = "express_country_entries_pt-br.json";
        let caps = RE.captures(file_name).unwrap();
        let lang_code = caps.get(1).unwrap().as_str();
        assert_eq!(lang_code, "pt-br");

        let new_file_name = file_name.replace(lang_code, "es");
        assert_eq!(new_file_name, "express_country_entries_es.json");
    }

    // cargo test -- --show-output test_entity_file_to_new_language
    #[test]
    fn test_entity_file_to_new_language() {
        let entity_entry = EntityEntry {
            value: "back".to_owned(),
            synonyms: vec!["rear".to_owned(), "tail end".to_owned()],
        };

        let entity_entries_file = EntityEntriesFile::new(
            "express_country_entries_en.json".to_owned(),
            vec![entity_entry.clone()],
        );
        let entity_entries_file_expected = EntityEntriesFile::new(
            "express_country_entries_pt-br.json".to_owned(),
            vec![entity_entry],
        );
        let cloned = entity_entries_file.to_new_language("pt-br");
        assert_eq!(cloned, entity_entries_file_expected);
    }

    // cargo test -- --show-output test_translation_mechanics
    #[test]
    fn test_translation_mechanics() {
        let item1 = DummyStructSlave::new("foo1".to_owned(), "bar1".to_owned(), "id1".to_owned());
        let item2 = DummyStructSlave::new("foo2".to_owned(), "bar2".to_owned(), "id2".to_owned());
        let items = vec![item1, item2];

        let mut master = DummyStructMaster::new(items);

        let master_iter = master.items.iter();

        let mut translation_map: collections::HashMap<String, String> = collections::HashMap::new();

        for item in master_iter {
            translation_map.extend(item.to_translation());
        }

        println!("before translation");
        for (_, text_to_translate) in &translation_map {
            println!("{}", text_to_translate);
        }
        println!("{:#?}", translation_map);
        println!("{:#?}", master);

        assert_eq!(master.items[0].foo, "foo1");
        assert_eq!(master.items[0].bar, "bar1");
        assert_eq!(master.items[1].foo, "foo2");
        assert_eq!(master.items[1].bar, "bar2");

        // translation will iterate (producing mutable values) over hashmap product and create translated texts
        for val in translation_map.values_mut() {
            let translated_text = format!("{}{}", val, "_translated!");
            *val = translated_text;
        }

        // then we will iterate (using mutable iterator) original structure and lookup translated values based on struct member pointer addresses
        let master_iter_mut = master.items.iter_mut();

        for item in master_iter_mut {
            item.from_translation(&translation_map);
        }

        println!("after translation");
        for (_, text_to_translate) in &translation_map {
            println!("{}", text_to_translate);
        }
        println!("{:#?}", translation_map);
        println!("{:#?}", master);

        assert_eq!(master.items[0].foo, "foo1_translated!");
        assert_eq!(master.items[0].bar, "bar1_translated!");
        assert_eq!(master.items[1].foo, "foo2_translated!");
        assert_eq!(master.items[1].bar, "bar2_translated!");
    }

    // cargo test -- --show-output test_translate_entity_entry
    #[test]
    fn test_translate_entity_entry() -> Result<()> {
        let entity_entry_str = r#"
            {
              "value": "beautiful",
              "synonyms": ["charming", "alluring", "lovely"]
            }
        "#;

        let entity_entry_str_translated_exptected = r#"
        {
            "value": "beautiful_translated",
            "synonyms": [
              "charming_translated",
              "alluring_translated",
              "lovely_translated"
            ]
          }
        "#;

        translation_tests_assertions!(
            EntityEntry,
            entity_entry_str,
            entity_entry_str_translated_exptected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_intent_utternce
    #[test]
    fn test_translate_intent_utternce() -> Result<()> {
        let utterance_str = r#"
        {
            "text": "when will I receive the ",
            "userDefined": false
        }
        "#;

        let utterance_str_translated_exptected = r#"
        {
            "text": "when will I receive the _translated",
            "userDefined": false
        }
        "#;

        translation_tests_assertions!(
            IntentUtteranceData,
            utterance_str,
            utterance_str_translated_exptected
        );
        Ok(())
    }

    // cargo test -- --show-output test_translate_intent_parameter_prompt
    #[test]
    fn test_translate_intent_parameter_prompt() -> Result<()> {
        let prompts_str = r#"
        {
            "lang": "en",
            "value": "What field are you working in?"
          }
        "#;

        let prompts_str_translated_exptected = r#"
        {
            "lang": "en",
            "value": "What field are you working in?_translated"
          }
        "#;

        translation_tests_assertions!(
            IntentResponseParameterPrompt,
            prompts_str,
            prompts_str_translated_exptected,
            "no_string_comparison"
        );
        Ok(())
    }

    #[test]
    fn test_entity_entry_file_name_to_entity_filename() {
        assert_eq!(
            GoogleDialogflowAgent::entity_entry_file_name_to_entity_filename(
                "sys.color_entries_en.json"
            ),
            "sys.color.json"
        );
        assert_eq!(
            GoogleDialogflowAgent::entity_entry_file_name_to_entity_filename(
                "PlacementLocationSide_entries_pt-br.json"
            ),
            "PlacementLocationSide.json"
        );
    }

    //
    // integration tests
    //

    #[test]
    #[ignore]
    fn test_unzip() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "FAQ.zip");
        let target_folder = "c:/tmp/z/unpacked";

        unzip_file(&path, target_folder)?;

        Ok(())
    }

    // running this test from VSCode will create folder in /target/debug folder
    // running from cmd line (see command below) will create folder in /target/debug/deps !
    // cargo test -- --show-output test_parse_gdf_agent_zip
    #[test]
    #[ignore]
    fn test_parse_gdf_agent_zip() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "FAQ.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        println!("{:#?}", agent);
        let map = agent.to_translation("en", "de");
        println!("{:#?}", map);
        Ok(())
    }

    // cargo test -- --show-output test_dummy_translate_agent
    #[test]
    #[ignore]
    fn test_dummy_translate_agent() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "FAQ.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        println!("agent before{:#?}", agent);
        let mut translation_map = agent.to_translation("en", "de");
        println!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        println!("translation_map after{:#?}", translation_map);

        agent.from_translation(&translation_map, "de");
        println!("agent after{:#?}", agent);
        Ok(())
    }

    // cargo test -- --show-output test_serialize_agent
    #[test]
    #[ignore]
    fn test_serialize_agent() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "FAQ.zip");
        let agent = parse_gdf_agent_zip(&path)?;
        agent.serialize("c:/tmp/out")?;
        Ok(())
    }

    // cargo test -- --show-output test_dummy_translate_and_serialize_agent_1
    #[test]
    //#[ignore]
    fn test_dummy_translate_and_serialize_agent_1() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Alarm.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // println!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        // println!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // println!("agent after{:#?}", agent);
        agent.serialize("c:/tmp/out/alarm")?;
        Ok(())
    }
    
    // cargo test -- --show-output test_dummy_translate_and_serialize_agent_2
    #[test]
    //#[ignore]
    fn test_dummy_translate_and_serialize_agent_2() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "App-Management.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // println!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        // println!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // println!("agent after{:#?}", agent);
        agent.serialize("c:/tmp/out/appmgmt")?;
        Ok(())
    }
    
    // cargo test -- --show-output test_dummy_translate_and_serialize_agent_3
    #[test]
    //#[ignore]
    fn test_dummy_translate_and_serialize_agent_3() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Banking.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // println!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        // println!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // println!("agent after{:#?}", agent);
        agent.serialize("c:/tmp/out/banking")?;
        Ok(())
    }
    
    // cargo test -- --show-output test_dummy_translate_and_serialize_agent_4
    #[test]
    //#[ignore]
    fn test_dummy_translate_and_serialize_agent_4() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Car.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // println!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        // println!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // println!("agent after{:#?}", agent);
        agent.serialize("c:/tmp/out/car")?;
        Ok(())
    }
    
    // cargo test -- --show-output test_dummy_translate_and_serialize_agent_5
    #[test]
    //#[ignore]
    fn test_dummy_translate_and_serialize_agent_5() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Coffee-Shop.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // println!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        // println!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // println!("agent after{:#?}", agent);
        agent.serialize("c:/tmp/out/coffeeshop")?;
        Ok(())
    }

    // cargo test -- --show-output test_dummy_translate_and_serialize_agent_6
    #[test]
    //#[ignore]
    fn test_dummy_translate_and_serialize_agent_6() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "FAQ.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // println!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        // println!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // println!("agent after{:#?}", agent);
        agent.serialize("c:/tmp/out/faq")?;
        Ok(())
    }

    // cargo test -- --show-output test_dummy_translate_and_serialize_agent_7
    #[test]
    //#[ignore]
    fn test_dummy_translate_and_serialize_agent_7() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Hotel-Booking.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // println!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        // println!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // println!("agent after{:#?}", agent);
        agent.serialize("c:/tmp/out/hotelbooking")?;
        Ok(())
    }
    
    // cargo test -- --show-output test_dummy_translate_and_serialize_agent_8
    #[test]
    //#[ignore]
    fn test_dummy_translate_and_serialize_agent_8() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Navigation.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // println!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        // println!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // println!("agent after{:#?}", agent);
        agent.serialize("c:/tmp/out/navigation")?;
        Ok(())
    }
    
    // cargo test -- --show-output test_dummy_translate_and_serialize_agent_9
    #[test]
    //#[ignore]
    fn test_dummy_translate_and_serialize_agent_9() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Smart-Home.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // println!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        // println!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // println!("agent after{:#?}", agent);
        agent.serialize("c:/tmp/out/smarthome")?;
        Ok(())
    }
    
    // cargo test -- --show-output test_dummy_translate_and_serialize_agent_10
    #[test]
    //#[ignore]
    fn test_dummy_translate_and_serialize_agent_10() -> Result<()> {
        let path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Support.zip");
        let mut agent = parse_gdf_agent_zip(&path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // println!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        // println!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // println!("agent after{:#?}", agent);
        agent.serialize("c:/tmp/out/support")?;
        Ok(())
    }    
}

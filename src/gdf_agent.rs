use crate::errors::{Error, Result};
use crate::gdf_responses::MessageType;
use assert_json_diff::assert_json_eq_no_panic;
use glob::glob;
use log::debug;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections;
use std::env::current_exe;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use zip;

pub trait Translate {
    fn to_translation(&self) -> collections::HashMap<String, String>;
    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>);
}

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EntityEntry {
    pub value: String,
    pub synonyms: Vec<String>,
}

impl Translate for EntityEntry {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        map_to_translate.insert(format!("{:p}", &self.value), self.value.to_owned());

        for synonym in self.synonyms.iter() {
            map_to_translate.insert(format!("{:p}", synonym), synonym.to_owned());
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.value = translations_map
            .get(&format!("{:p}", &self.value))
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

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentEvent {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentResponseAffectedContext {
    pub name: String,
    pub parameters: collections::HashMap<String, String>, // ??
    pub lifespan: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentResponseParameter {
    id: String,
    required: bool,

    #[serde(rename = "dataType")]
    data_type: String,

    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,

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

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentResponse {
    #[serde(rename = "resetContexts")]
    pub reset_contexts: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    #[serde(rename = "affectedContexts")]
    pub affected_contexts: Vec<IntentResponseAffectedContext>,

    pub parameters: Vec<IntentResponseParameter>,

    pub messages: Vec<MessageType>,

    #[serde(rename = "defaultResponsePlatforms")]
    pub default_response_platforms: collections::HashMap<String, bool>,

    pub speech: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Intent {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rootParentId")]
    pub root_parent_id: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IntentUtteranceData {
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<String>,
    #[serde(rename = "userDefined")]
    user_defined: bool,
}

impl Translate for IntentUtteranceData {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentUtterance {
    pub id: String,
    pub data: Vec<IntentUtteranceData>,

    #[serde(rename = "isTemplate")]
    pub is_template: bool,
    pub count: i8,
    pub updated: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityFile {
    pub file_name: String,
    pub file_content: Entity,
}

impl EntityFile {
    fn new(file_name: String, file_content: Entity) -> Self {
        EntityFile {
            file_name,
            file_content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityEntriesFile {
    pub file_name: String,
    pub file_content: Vec<EntityEntry>,
}

impl EntityEntriesFile {
    fn new(file_name: String, file_content: Vec<EntityEntry>) -> Self {
        EntityEntriesFile {
            file_name,
            file_content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentFile {
    pub file_name: String,
    pub file_content: Intent,
}

impl IntentFile {
    fn new(file_name: String, file_content: Intent) -> Self {
        IntentFile {
            file_name,
            file_content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentUtterancesFile {
    pub file_name: String,
    pub file_content: Vec<IntentUtterance>,
}

impl IntentUtterancesFile {
    fn new(file_name: String, file_content: Vec<IntentUtterance>) -> Self {
        IntentUtterancesFile {
            file_name,
            file_content,
        }
    }
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
}

pub fn unzip_file(zip_path: &str, target_folder: &str) -> Result<()> {
    let fname = std::path::Path::new(zip_path);
    let file = fs::File::open(&fname)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let base_path = Path::new(target_folder);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = base_path.join(file.sanitized_name());

        {
            let comment = file.comment();
            if !comment.is_empty() {
                debug!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            debug!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            fs::create_dir_all(&outpath)?;
        } else {
            debug!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

// not used in the end, if we actually wanted to put somewhere deserialized that generics would become
// quite cumbersome and nasty using macro instead
#[allow(dead_code)]
fn check_gdf_zip_glob_files<T>(glob_exp: &str, contains_array: bool) -> Result<()>
where
    T: DeserializeOwned + Serialize, // see https://serde.rs/lifetimes.html !
{
    for entry in glob(glob_exp)? {
        let path = entry?;

        let file_name = path.as_path().to_str().unwrap();

        if contains_array == false
            && (file_name.contains("_entries_") || file_name.contains("_usersays_"))
        {
            continue; // if not processing arrays (entity entries or intent utterances) skip respective files!
        }

        debug!("processing file {}", file_name);
        let file_str = fs::read_to_string(file_name)?;

        let deserialized_struct: T = serde_json::from_str(&file_str)?;

        let serialized_str = serde_json::to_string(&deserialized_struct).unwrap();
        let comparison_result = assert_json_eq_no_panic(
            &serde_json::from_str(&serialized_str)?,
            &serde_json::from_str(&file_str)?,
        );

        if let Err(err_msg) = comparison_result {
            return Err(Error::new(err_msg));
        }
    }
    Ok(())
}

// definying this function with generics is quite tricky becase of calling <<DeserializedStructType>>::new
// macri is good way here how to prevent writing same function4 times
macro_rules! parse_gdf_agent_files {
    ($name:ident, $type_deserialized:ty, $type_output:ty) => {
        fn $name(glob_exp: &PathBuf) -> Result<Vec<$type_output>> {
            let mut output_vec: Vec<$type_output> = vec![];
            let glob_str = glob_exp.as_path().to_str().unwrap();
            debug!(
                "entering parse_gdf_agent_files macro with glob_str {}",
                glob_str
            );
            for entry in glob(glob_str)? {
                let path = entry?;

                let file_name = path.as_path().to_str().unwrap();
                debug!("going to process file {}", file_name);

                // if not processing arrays (entity entries or intent utterances) skip
                // respective files (which are otherwise include in glob expresion)!
                if !glob_str.contains("_*.json")
                    && (file_name.contains("_entries_") || file_name.contains("_usersays_"))
                {
                    debug!("skipping the processing of the file file {}", file_name);
                    continue; // if not processing arrays (entity entries or intent utterances) skip respective files!
                }

                debug!("processing file {}", file_name);
                let file_str = fs::read_to_string(file_name)?;

                let deserialized_struct: $type_deserialized = serde_json::from_str(&file_str)?;

                let serialized_str = serde_json::to_string(&deserialized_struct).unwrap();
                let comparison_result = assert_json_eq_no_panic(
                    &serde_json::from_str(&serialized_str)?,
                    &serde_json::from_str(&file_str)?,
                );

                if let Err(err_msg) = comparison_result {
                    return Err(Error::new(err_msg));
                }
                output_vec.push(<$type_output>::new(
                    file_name.to_string(),
                    deserialized_struct,
                ));
            }
            Ok(output_vec)
        }
    };
}

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

pub fn parse_gdf_agent_zip(zip_path: &str) -> Result<GoogleDialogflowAgent> {
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
    use crate::gdf_responses::normalize_json;
    use assert_json_diff::assert_json_eq;

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

    fn dummy_translate(translation_map: &mut collections::HashMap<String, String>) {
        for val in translation_map.values_mut() {
            let translated_text = format!("{}{}", val, "_translated");
            *val = translated_text;
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
    // speech with single string specified  + aeeay of responses for response message
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

        let mut entry: EntityEntry = serde_json::from_str(entity_entry_str)?;
        let entry_translated: EntityEntry =
            serde_json::from_str(entity_entry_str_translated_exptected)?;
        let mut translations_map = entry.to_translation();

        dummy_translate(&mut translations_map);
        entry.from_translation(&translations_map);
        let entity_entry_str_translated = serde_json::to_string(&entry)?;

        assert_eq!(
            normalize_json(&entity_entry_str_translated),
            normalize_json(&entity_entry_str_translated_exptected)
        );

        assert_eq!(entry, entry_translated);
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

        let mut utterance: IntentUtteranceData = serde_json::from_str(utterance_str)?;
        let utterance_translated: IntentUtteranceData =
            serde_json::from_str(utterance_str_translated_exptected)?;
        let mut translations_map = utterance.to_translation();

        dummy_translate(&mut translations_map);
        utterance.from_translation(&translations_map);
        let utterance_str_translated = serde_json::to_string(&utterance)?;

        assert_eq!(
            normalize_json(&utterance_str_translated),
            normalize_json(&utterance_str_translated_exptected)
        );

        assert_eq!(utterance, utterance_translated);
        Ok(())
    }    

    //
    // integration tests
    //

    #[test]
    #[ignore]
    fn test_unzip() -> Result<()> {
        let path = "c:/tmp/z/Express_CS_AM_PRD.zip";
        let target_folder = "c:/tmp/z/unpacked";

        unzip_file(path, target_folder)?;

        Ok(())
    }

    // running this test from VSCode will create folder in /target/debug folder
    // running from cmd line (see command below) will create folder in /target/debug/deps !
    // cargo test -- --show-output test_parse_gdf_agent_zip
    #[test]
    #[ignore]
    fn test_parse_gdf_agent_zip() -> Result<()> {
        let path = "c:/tmp/z/Express_CS_AM_PRD.zip";
        let _agent = parse_gdf_agent_zip(path)?;
        // println!("{:#?}", agent);
        Ok(())
    }
}

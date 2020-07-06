use crate::google::dialogflow::agent::Translate;
use crate::google::dialogflow::responses::MessageType;
use serde::{Deserialize, Serialize};
use std::collections;

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct IntentResponseParameterPrompt {
    lang: String,
    value: String,
}

impl Translate for IntentResponseParameterPrompt {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();
        map_to_translate.insert(format!("{:p}", &self.value), self.value.to_owned());
        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        self.value = translations_map
            .get(&format!("{:p}", &self.value))
            .unwrap()
            .to_owned();
    }
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

    #[serde(skip_serializing_if = "Option::is_none")]
    prompts: Option<Vec<IntentResponseParameterPrompt>>,

    #[serde(rename = "promptMessages")]
    prompt_messages: Vec<String>,

    #[serde(rename = "noMatchPromptMessages")]
    no_match_prompt_messages: Vec<String>,

    #[serde(rename = "noInputPromptMessages")]
    no_input_prompt_messages: Vec<String>,

    #[serde(rename = "outputDialogContexts")]
    output_dialog_contexts: Vec<String>,

    // see Smart-Home example, intent smarthome.locks.open.json.
    // Not sure hwo to set this up in DialogFlow UI and whether we  should actuallyt ranslate it
    // For now just adding here so that we can properly deserialize and serialize back, not translating
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultValue")]
    default_value: Option<String>,

    #[serde(rename = "isList")]
    is_list: bool,
}

impl Translate for IntentResponseParameter {
    fn to_translation(&self) -> collections::HashMap<String, String> {
        let mut map_to_translate = collections::HashMap::new();

        if let Some(prompts) = &self.prompts {
            for prompt in prompts.iter() {
                map_to_translate.extend(prompt.to_translation());
            }
        }

        map_to_translate
    }

    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>) {
        if let Some(prompts) = &mut self.prompts {
            for prompt in prompts.iter_mut() {
                prompt.from_translation(translations_map);
            }
        }
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentFile {
    pub file_name: String,
    pub file_content: Intent,
}

impl IntentFile {
    pub fn new(file_name: String, file_content: Intent) -> Self {
        IntentFile {
            file_name,
            file_content,
        }
    }
}

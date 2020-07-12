use crate::errors::Result;
use crate::google::dialogflow::agent::parse_gdf_agent_zip;
use log::debug;
use std::collections;

pub mod v2;
pub mod v3;

// TBD make this trait async (see https://crates.io/crates/async-trait)
pub trait TranslationFlow {
    fn execute_translation(gdf_agent_path: &str, translated_gdf_agent_folder: &str) -> Result<()>;
    // TBD: remove this method from agent.rs and refactor it to use TranslationFlow::dummy_translate instead!
    fn dummy_translate(translation_map: &mut collections::HashMap<String, String>) {
        for val in translation_map.values_mut() {
            let translated_text = format!("{}{}", val, "_translated");
            *val = translated_text;
        }
    }
}

pub enum TranslationProviders {
    GoogleTranslateV2,
    GoogleTranslateV3,
    DummyTranslate,
}

pub struct GoogleTranslateV2;
pub struct GoogleTranslateV3;
pub struct DummyTranslate;

#[allow(unused_variables)]
impl TranslationFlow for GoogleTranslateV2 {
    fn execute_translation(gdf_agent_path: &str, translated_gdf_agent_folder: &str) -> Result<()> {
        // TBD...
        Ok(())
    }
}

#[allow(unused_variables)]
impl TranslationFlow for GoogleTranslateV3 {
    fn execute_translation(gdf_agent_path: &str, translated_gdf_agent_folder: &str) -> Result<()> {
        // TBD...
        Ok(())
    }
}

#[allow(unused_variables)]
impl TranslationFlow for DummyTranslate {
    fn execute_translation(gdf_agent_path: &str, translated_gdf_agent_folder: &str) -> Result<()> {
        debug!("processing agent {}", gdf_agent_path);
        let mut agent = parse_gdf_agent_zip(gdf_agent_path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // debug!("translation_map before{:#?}", translation_map);
        DummyTranslate::dummy_translate(&mut translation_map);
        // debug!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // debug!("agent after{:#?}", agent);
        agent.serialize(translated_gdf_agent_folder)?;
        Ok(())
    }
}

pub fn get_translation_provider(
    provider_type: TranslationProviders,
) -> fn(&str, &str) -> Result<()> {
    match provider_type {
        TranslationProviders::DummyTranslate => DummyTranslate::execute_translation,
        TranslationProviders::GoogleTranslateV2 => GoogleTranslateV2::execute_translation,
        TranslationProviders::GoogleTranslateV3 => GoogleTranslateV3::execute_translation,
    }
}

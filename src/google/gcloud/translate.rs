use crate::errors::Result;
use crate::google::dialogflow::agent::parse_gdf_agent_zip;
use log::debug;
use std::collections;

pub mod v2;
pub mod v3;

/// This trait is implemented by all agent's structs that should be translated
pub trait Translate {
    /// for given struct representing part of GDF agent creates
    /// translation map to be merged into master translation map
    fn to_translation(&self) -> collections::HashMap<String, String>;

    /// from master translation map retrieves respective translated entry
    fn from_translation(&mut self, translations_map: &collections::HashMap<String, String>);
}

/// dummy translation method which just adds _translated postfix to every text that should be translated
pub fn dummy_translate(translation_map: &mut collections::HashMap<String, String>) {
    for val in translation_map.values_mut() {
        let translated_text = format!("{}{}", val, "_translated");
        *val = translated_text;
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
impl GoogleTranslateV2 {
    #[allow(dead_code)]
    async fn execute_translation(
        gdf_agent_path: &str,
        translated_gdf_agent_folder: &str,
    ) -> Result<()> {
        // TBD...
        Ok(())
    }
}

#[allow(unused_variables)]
impl GoogleTranslateV3 {
    #[allow(dead_code)]
    async fn execute_translation(
        gdf_agent_path: &str,
        translated_gdf_agent_folder: &str,
    ) -> Result<()> {
        // TBD...
        Ok(())
    }
}

#[allow(unused_variables)]
impl DummyTranslate {
    #[allow(dead_code)]
    async fn execute_translation(
        gdf_agent_path: &str,
        translated_gdf_agent_folder: &str,
    ) -> Result<()> {
        debug!("processing agent {}", gdf_agent_path);
        let mut agent = parse_gdf_agent_zip(gdf_agent_path)?;
        let mut translation_map = agent.to_translation("en", "de");
        // debug!("translation_map before{:#?}", translation_map);
        dummy_translate(&mut translation_map);
        // debug!("translation_map after{:#?}", translation_map);
        agent.from_translation(&translation_map, "de");
        // debug!("agent after{:#?}", agent);
        agent.serialize(translated_gdf_agent_folder)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::task;

    const SAMPLE_AGENTS_FOLDER: &str =
        "C:/Users/abezecny/adam/WORK/_DEV/Rust/gdf_translate/examples/sample_agents/";

    // cargo test -- --show-output test_execute_translation_dummy
    #[test]
    #[ignore]
    fn test_execute_translation_dummy() -> Result<()> {
        let agent_path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Currency-Converter.zip");

        let _ = task::block_on(DummyTranslate::execute_translation(
            &agent_path,
            "c:/tmp/out_translated",
        ));

        Ok(())
    }
}

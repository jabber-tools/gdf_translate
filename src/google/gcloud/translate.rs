use crate::errors::{Error, Result};
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
        token: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<()> {
        debug!("processing agent {}", gdf_agent_path);
        let mut agent = parse_gdf_agent_zip(gdf_agent_path)?;
        let mut translation_map = agent.to_translation(source_lang, target_lang);

        for val in translation_map.values_mut() {
            debug!("translating value {}", *val);
            let translation_response = v2::translate(
                token,
                source_lang,
                target_lang,
                val,
                v2::TranslateFormat::Plain,
            )
            .await?;

            debug!("translation_response {:#?}", translation_response);

            if translation_response.status != "200" {
                return Err(Error::new(format!(
                    "GoogleTranslateV2.execute_translation error {:#?}",
                    translation_response
                )));
            }

            *val = translation_response
                .body
                .data
                .translations
                .iter()
                .map(|x| x.translated_text.to_owned())
                .collect::<Vec<String>>()
                .join(""); /**/
        }

        agent.from_translation(&translation_map, target_lang);
        agent.serialize(translated_gdf_agent_folder)?;
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
        dummy_translate(&mut translation_map);
        agent.from_translation(&translation_map, "de");
        agent.serialize(translated_gdf_agent_folder)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::google::gcloud::auth::*;
    use crate::init_logging; // set RUST_LOG=gdf_translate::google::gcloud::translate=debug
    use async_std::task;

    const SAMPLE_AGENTS_FOLDER: &str =
        "C:/Users/abezecny/adam/WORK/_DEV/Rust/gdf_translate/examples/sample_agents/";

    // cargo test -- --show-output test_execute_translation_dummy
    #[test]
    //#[ignore]
    fn test_execute_translation_dummy() -> Result<()> {
        init_logging();
        let agent_path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Currency-Converter.zip");
        println!("getting bearer token...");
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);
        println!("bearer token retrieved {}", token);

        let _ = task::block_on(DummyTranslate::execute_translation(
            &agent_path,
            "c:/tmp/out_translated",
        ));

        Ok(())
    }

    // cargo test -- --show-output test_execute_translation_google_v2
    #[test]
    //#[ignore]
    fn test_execute_translation_google_v2() -> Result<()> {
        init_logging();
        let agent_path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Currency-Converter.zip");
        println!("getting bearer token...");
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);
        println!("bearer token retrieved {}", token);
        let _ = task::block_on(GoogleTranslateV2::execute_translation(
            &agent_path,
            "c:/tmp/out_translated",
            &token,
            "en",
            "de",
        ));

        Ok(())
    }
}

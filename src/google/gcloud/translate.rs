use crate::errors::{Error, Result};
use crate::google::dialogflow::agent::parse_gdf_agent_zip;
use crate::google::gcloud::storage_bucket_mgmt;
use async_std::task;
use log::debug;
use std::collections;
use std::time::Duration;
use std::time::SystemTime;
// uncoment when bucket result file for debuging is enabled again
// use std::fs::File;
// use std::io::prelude::*;

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

        let translation_count = translation_map.len();
        let mut translated_item_idx = 0;

        for val in translation_map.values_mut() {
            translated_item_idx = translated_item_idx + 1;
            debug!(
                "translating value({}/{}): {}",
                translated_item_idx, translation_count, *val
            );
            let mut translation_response;
            let mut translation_result = v2::translate(
                token,
                source_lang,
                target_lang,
                val,
                v2::TranslateFormat::Plain,
            )
            .await;

            if let Err(translation_error) = translation_result {
                debug!(
                    "error while translating value {}/{}. Attempting one more time. Error detail: {:#?}",
                    translated_item_idx, translation_count, translation_error
                );
                task::sleep(Duration::from_secs(2)).await; // wait with this task execution before next try!
                translation_result = v2::translate(
                    token,
                    source_lang,
                    target_lang,
                    val,
                    v2::TranslateFormat::Plain,
                )
                .await;
            }

            if let Err(translation_error) = translation_result {
                debug!(
                    "2nd error while translating value {}/{}. Skipping translating of this item. Error detail: {:#?}",
                    translated_item_idx, translation_count, translation_error
                );
                continue;
            } else {
                debug!(
                    "call translation api for item {}/{} succeeded!",
                    translated_item_idx, translation_count
                );
                translation_response = translation_result.unwrap();
            }

            debug!("translation_response {:#?}", translation_response);

            if translation_response.status != "200" {
                debug!(
                    "error while translating value {}/{}. HTTP code is not 200. Attempting one more time. Error detail: {:#?}",
                    translated_item_idx, translation_count, translation_response
                );
                task::sleep(Duration::from_secs(2)).await; // wait with this task execution before next try!
                translation_result = v2::translate(
                    token,
                    source_lang,
                    target_lang,
                    val,
                    v2::TranslateFormat::Plain,
                )
                .await;

                if let Err(translation_error) = translation_result {
                    debug!(
                        "2nd error while translating value {}/{}. Skipping translating of this item. Error detail: {:#?}",
                        translated_item_idx, translation_count, translation_error
                    );
                    continue;
                }

                if translation_response.status != "200" {
                    debug!(
                        "2nd error while translating value {}/{}. HTTP code is not 200. Skipping translating of this item. Error detail: {:#?}",
                        translated_item_idx, translation_count, translation_response
                    );
                    continue;
                }

                debug!(
                    "2nd attempt to translate item {}/{} succeeded!",
                    translated_item_idx, translation_count
                );
                translation_response = translation_result.unwrap();
            }

            *val = translation_response
                .body
                .data
                .translations
                .iter()
                .map(|x| x.translated_text.to_owned())
                .collect::<Vec<String>>()
                .join("");
        }

        debug!("translation finished. updated translation map");
        debug!("{:#?}", translation_map);

        debug!("applying translated map to agent");
        agent.from_translation(&translation_map, target_lang);
        debug!("serializing agent");
        agent.serialize(translated_gdf_agent_folder)?;
        debug!("agent serialized!");
        Ok(())
    }
}

#[allow(unused_variables)]
impl GoogleTranslateV3 {
    #[allow(dead_code)]
    async fn execute_translation(
        gdf_agent_path: &str,
        translated_gdf_agent_folder: &str,
        token: &str,
        source_lang: &str,
        target_lang: &str,
        project_id: &str,
    ) -> Result<()> {
        debug!("processing agent {}", gdf_agent_path);
        let mut agent = parse_gdf_agent_zip(gdf_agent_path)?;

        let translation_map = agent.to_translation(source_lang, target_lang);
        debug!("translation_map {:#?}", translation_map);

        let ts_sec = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let storage_bucket_name_in = format!("gdf_translate_input_{}", ts_sec.to_string());
        let storage_bucket_name_out = format!("gdf_translate_output_{}", ts_sec.to_string());

        let bucket_creation_result_in = storage_bucket_mgmt::create_bucket(
            token,
            project_id,
            &storage_bucket_name_in,
            "EUROPE-WEST3",
            "STANDARD",
        )
        .await?;
        debug!("bucket_creation_result_in {:#?}", bucket_creation_result_in);

        let bucket_creation_result_out = storage_bucket_mgmt::create_bucket(
            token,
            project_id,
            &storage_bucket_name_out,
            "EUROPE-WEST3",
            "STANDARD",
        )
        .await?;
        debug!(
            "bucket_creation_result_out {:#?}",
            bucket_creation_result_out
        );

        if bucket_creation_result_out.status_code != "200" {
            return Err(Error::new(format!(
                "GoogleTranslateV3.execute_translation error when creating bucket {:#?}",
                bucket_creation_result_out
            )));
        }

        let map_str = v3::map_to_string(&translation_map);
        debug!("v3::map_to_string:\n {}", map_str);

        let bucket_upload_result = storage_bucket_mgmt::upload_object(
            token,
            &storage_bucket_name_in,
            "translation_map.tsv",
            &map_str,
        )
        .await?;
        debug!("bucket_upload_result {:#?}", bucket_upload_result);

        if bucket_upload_result.status_code != "200" {
            return Err(Error::new(format!(
                "GoogleTranslateV3.execute_translation error when uploading bucket {:#?}",
                bucket_upload_result
            )));
        }

        let translation_result = v3::batch_translate_text(
            token,
            project_id,
            source_lang,
            target_lang,
            "text/html", // alwats HTML, we are wrapping text to translate in <span> tag
            &format!("gs://{}/translation_map.tsv", storage_bucket_name_in),
            &format!("gs://{}/", storage_bucket_name_out),
        )
        .await?;
        debug!("translation_result {:#?}", translation_result);

        if translation_result.status_code != "200" {
            return Err(Error::new(format!(
                "GoogleTranslateV3.execute_translation error when starting batch translation {:#?}",
                translation_result
            )));
        }

        loop {
            let translation_operation_result =
                v3::batch_translate_text_check_status(token, &translation_result.body.name).await?;

            debug!(
                "translation_operation_result {:#?}",
                translation_operation_result
            );

            if translation_operation_result.status_code != "200" {
                return Err(Error::new(format!(
                    "GoogleTranslateV3.execute_translation error when checking long running operation {:#?}",
                    translation_operation_result
                )));
            }

            if let Some(done) = translation_operation_result.body.done {
                if done == true && translation_operation_result.body.metadata.state == "SUCCEEDED" {
                    debug!("batch translation completed!");
                    break;
                } else
                /* FAILED*/
                {
                    if let Some(error) = translation_operation_result.body.error {
                        debug!("batch translation failed! Error detail {:#?}", error);
                        return Err(Error::new(format!(
                            "GoogleTranslateV3.execute_translation failed {:#?}",
                            error
                        )));
                    }
                }
            } else {
                debug!("still running, checking the state again...")
            }
        }

        // e.g. gs://gdf_translate_output_1594998623/gdf_translate_input_1594998623_translation_map_de_translations.tsv
        let translated_object_name = format!(
            "{}_translation_map_{}_translations.tsv",
            storage_bucket_name_in, target_lang
        );

        debug!("translated_object_name {}", translated_object_name);

        let bucket_download_result = storage_bucket_mgmt::download_object(
            token,
            &storage_bucket_name_out,
            &translated_object_name,
        )
        .await?;
        debug!("bucket_download_result {:#?}", bucket_download_result);

        if bucket_download_result.status_code != "200" {
            return Err(Error::new(format!(
                "GoogleTranslateV3.execute_translation error downloading translation results {:#?}",
                bucket_download_result
            )));
        }

        //
        // just for debugging, disable then
        //
        // let mut file_handle = File::create(format!("{}/bucket_download_result.txt", translated_gdf_agent_folder))?;
        // file_handle.write_all(bucket_download_result.body.as_bytes())?;

        let translated_map = v3::string_to_map(bucket_download_result.body)?;

        let mut delete_object_result;

        debug!("deleting index.csv");
        delete_object_result =
            storage_bucket_mgmt::delete_object(token, &storage_bucket_name_out, "index.csv")
                .await?;
        debug!("delete_object_result {:#?}", delete_object_result);

        debug!("deleting {}", translated_object_name);
        delete_object_result = storage_bucket_mgmt::delete_object(
            token,
            &storage_bucket_name_out,
            &translated_object_name,
        )
        .await?;
        debug!("delete_object_result {:#?}", delete_object_result);

        debug!("deleting translation_map.tsv");
        delete_object_result = storage_bucket_mgmt::delete_object(
            token,
            &storage_bucket_name_in,
            "translation_map.tsv",
        )
        .await?;
        debug!("delete_object_result {:#?}", delete_object_result);

        debug!("deleting {}", &storage_bucket_name_in);
        let delete_bucket_result_in =
            storage_bucket_mgmt::delete_bucket(token, &storage_bucket_name_in).await?;
        debug!("delete_bucket_result_in {:#?}", delete_bucket_result_in);

        debug!("deleting {}", &storage_bucket_name_out);
        let delete_bucket_result_out =
            storage_bucket_mgmt::delete_bucket(token, &storage_bucket_name_out).await?;
        debug!("delete_bucket_result_out {:#?}", delete_bucket_result_out);

        debug!("translation finished. updated translation map");
        debug!("{:#?}", translated_map);

        debug!("applying translated map to agent");
        agent.from_translation(&translated_map, target_lang);
        debug!("serializing agent");
        agent.serialize(translated_gdf_agent_folder)?;
        debug!("agent serialized!");

        Ok(())
    }
}

#[allow(unused_variables)]
impl DummyTranslate {
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
        dummy_translate(&mut translation_map);
        agent.from_translation(&translation_map, target_lang);
        agent.serialize(translated_gdf_agent_folder)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::google::gcloud::auth::*;
    use crate::init_logging; // set RUST_LOG=gdf_translate::google::gcloud::translate=debug

    const SAMPLE_AGENTS_FOLDER: &str =
        "C:/Users/abezecny/adam/WORK/_DEV/Rust/gdf_translate/examples/sample_agents/";

    // cargo test -- --show-output test_execute_translation_dummy
    #[test]
    #[ignore]
    fn test_execute_translation_dummy() -> Result<()> {
        init_logging();
        let agent_path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Currency-Converter.zip");
        debug!("getting bearer token...");
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);
        debug!("bearer token retrieved {}", token);

        let _ = task::block_on(DummyTranslate::execute_translation(
            &agent_path,
            "c:/tmp/out_translated",
            "token n/a",
            "en",
            "de",
        ));

        Ok(())
    }

    // cargo test -- --show-output test_execute_translation_google_v2
    #[test]
    #[ignore]
    fn test_execute_translation_google_v2() -> Result<()> {
        init_logging();
        let agent_path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Currency-Converter.zip");
        debug!("getting bearer token...");
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);
        debug!("bearer token retrieved {}", token);
        let _ = task::block_on(GoogleTranslateV2::execute_translation(
            &agent_path,
            "c:/tmp/out_translated",
            &token,
            "en",
            "de",
        ));

        Ok(())
    }

    // cargo test -- --show-output test_execute_translation_google_v3
    #[test]
    //#[ignore]
    fn test_execute_translation_google_v3() -> Result<()> {
        init_logging();
        let agent_path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Currency-Converter.zip");
        debug!("getting bearer token...");
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);
        debug!("bearer token retrieved {}", token);
        let _ = task::block_on(GoogleTranslateV3::execute_translation(
            &agent_path,
            "c:/tmp/out_translated",
            &token,
            "en",
            "de",
            "express-tracking",
        ));

        Ok(())
    }
}

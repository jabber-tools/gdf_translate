use crate::errors::{Error, Result};
use crate::google::dialogflow::agent::parse_gdf_agent_zip;
use crate::google::gcloud::storage_bucket_mgmt;
use crate::ui::ProgressMessageType;
use async_std::task;
// while StreamExt is not used directly without it this line will not compile:
// while let Some(future_value) = futures.next().await
use futures::stream::{FuturesUnordered, StreamExt};
use log::debug;
use std::collections;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::time::SystemTime;

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

fn send_progress(msg: ProgressMessageType, mpsc_sender: &Sender<ProgressMessageType>) {
    mpsc_sender.send(msg).unwrap();
}

#[derive(Debug)]
pub enum TranslationProviders {
    GoogleTranslateV2,
    GoogleTranslateV3,
    DummyTranslate,
}

pub struct GoogleTranslateV2;
pub struct GoogleTranslateV3;
pub struct DummyTranslate;

impl GoogleTranslateV2 {
    pub fn execute_translation(
        gdf_agent_path: &str,
        translated_gdf_agent_folder: &str,
        token: &str,
        source_lang: &str,
        target_lang: &str,
        mpsc_sender: Sender<ProgressMessageType>,
        task_count: usize,
        skip_entities_translation: bool,
        skip_utterances_translation: bool,
        skip_responses_translation: bool,
    ) -> Result<()> {
        debug!("processing agent {}", gdf_agent_path);
        let mut agent = parse_gdf_agent_zip(gdf_agent_path)?;
        let mut translation_map = agent.to_translation(
            source_lang,
            target_lang,
            skip_entities_translation,
            skip_utterances_translation,
            skip_responses_translation,
        );

        let translation_count = translation_map.len();
        send_progress(
            ProgressMessageType::CountSpecified(translation_count as u64),
            &mpsc_sender,
        );

        let mut translation_maps: Vec<collections::HashMap<String, String>> = Vec::new();
        send_progress(
            ProgressMessageType::TextMessage("partitioning translation map".to_owned()),
            &mpsc_sender,
        );

        let submap_item_count = translation_count / task_count;
        let mut item_counter = 0;
        translation_maps.push(collections::HashMap::new());
        for (k, v) in translation_map.drain() {
            item_counter = item_counter + 1;
            let mut maps_length = translation_maps.len();
            if item_counter > submap_item_count {
                translation_maps.push(collections::HashMap::new());
                maps_length = translation_maps.len();
                item_counter = 0;
            }
            translation_maps[maps_length - 1].insert(k, v);
        }

        let mut futures = FuturesUnordered::new();

        let mut iter_idx = 0;
        while let Some(map) = translation_maps.pop() {
            iter_idx = iter_idx + 1;
            let future = GoogleTranslateV2::execute_translation_impl(
                token,
                source_lang,
                target_lang,
                map,
                mpsc_sender.clone(),
                iter_idx,
            );
            futures.push(future);
        }

        // inspired by https://www.philipdaniels.com/blog/2019/async-std-demo1/
        // https://users.rust-lang.org/t/futuresunordered/39461/4
        // uses asynchronous streams (see next() method), unfortunatelly this is still not described in async-std documentation (see https://book.async.rs/concepts/streams.html)
        // so it required little bit of investigation, not sure whether this is optimal way, probably tokio has better (and better documented) capabilities when it comes to joining the futures
        task::block_on(async {
            while let Some(future_value) = futures.next().await {
                // for this to compile StreamExt must be used, see use futures::stream::{FuturesUnordered, StreamExt}; !
                match future_value {
                    Ok(translated_submap) => translation_map.extend(translated_submap),
                    Err(e) => debug!(" Error when resolving future returned by GoogleTranslateV2::execute_translation_impl : {:#?}", e),
                    // TBD: emit some text to CLI, maybe terminate the processing?
                }
            }
        });

        debug!("translation finished. updated translation map");
        debug!("{:#?}", translation_map);

        debug!("applying translated map to agent");
        agent.from_translation(
            &translation_map,
            target_lang,
            skip_entities_translation,
            skip_utterances_translation,
            skip_responses_translation,
        );
        agent.add_supported_language(target_lang);
        debug!("serializing agent");
        agent.serialize(translated_gdf_agent_folder)?;
        debug!("agent serialized!");

        send_progress(ProgressMessageType::Exit, &mpsc_sender);

        Ok(())
    }

    async fn execute_translation_impl(
        token: &str,
        source_lang: &str,
        target_lang: &str,
        mut translation_map: collections::HashMap<String, String>,
        mpsc_sender: Sender<ProgressMessageType>,
        iter_idx: usize,
    ) -> Result<collections::HashMap<String, String>> {
        let translation_count = translation_map.len();
        let mut translated_item_idx = 0;
        for val in translation_map.values_mut() {
            translated_item_idx = translated_item_idx + 1;
            debug!(
                "translating value({}/{} for sub-batch: {}): {}",
                translated_item_idx, translation_count, *val, iter_idx
            );
            send_progress(ProgressMessageType::ItemProcessed, &mpsc_sender);

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
                    "error while translating value {}/{} for sub-batch: {}. Attempting one more time. Error detail: {:#?}",
                    translated_item_idx, translation_count, iter_idx, translation_error
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
                    "2nd error while translating value {}/{} for sub-batch: {}. Skipping translating of this item. Error detail: {:#?}",
                    translated_item_idx, translation_count, iter_idx, translation_error
                );
                continue;
            } else {
                debug!(
                    "call translation api for item {}/{} for sub-batch: {} succeeded!",
                    translated_item_idx, translation_count, iter_idx
                );
                translation_response = translation_result.unwrap();
            }

            debug!("translation_response {:#?}", translation_response);

            if translation_response.status != "200" {
                debug!(
                    "error while translating value {}/{} for sub-batch {}. HTTP code is not 200. Attempting one more time. Error detail: {:#?}",
                    translated_item_idx, translation_count, iter_idx, translation_response
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
                        "2nd error while translating value {}/{} for sub-batch {}. Skipping translating of this item. Error detail: {:#?}",
                        translated_item_idx, translation_count, iter_idx, translation_error
                    );
                    continue;
                }

                if translation_response.status != "200" {
                    debug!(
                        "2nd error while translating value {}/{} for sub-batch {}. HTTP code is not 200. Skipping translating of this item. Error detail: {:#?}",
                        translated_item_idx, translation_count, iter_idx, translation_response
                    );
                    continue;
                }

                debug!(
                    "2nd attempt to translate item {}/{} for sub-batch {} succeeded!",
                    translated_item_idx, translation_count, iter_idx
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
        Ok(translation_map)
    }
}

impl GoogleTranslateV3 {
    pub async fn execute_translation(
        gdf_agent_path: &str,
        translated_gdf_agent_folder: &str,
        token: &str,
        source_lang: &str,
        target_lang: &str,
        project_id: &str,
        mpsc_sender: Sender<ProgressMessageType>,
        create_output_tsv: bool,
        skip_entities_translation: bool,
        skip_utterances_translation: bool,
        skip_responses_translation: bool,
    ) -> Result<()> {
        debug!("processing agent {}", gdf_agent_path);
        send_progress(
            ProgressMessageType::TextMessage("parsing zip file".to_owned()),
            &mpsc_sender,
        );
        let mut agent = parse_gdf_agent_zip(gdf_agent_path)?;

        send_progress(
            ProgressMessageType::TextMessage("preparing translation map".to_owned()),
            &mpsc_sender,
        );
        let mut translation_map = agent.to_translation(
            source_lang,
            target_lang,
            skip_entities_translation,
            skip_utterances_translation,
            skip_responses_translation,
        );
        debug!("translation_map {:#?}", translation_map);

        // partitioning translation map into subsets due to limitation / quotas of Google Translate V3 API
        let mut translation_maps: Vec<collections::HashMap<String, String>> = Vec::new();
        send_progress(
            ProgressMessageType::TextMessage("partitioning translation map".to_owned()),
            &mpsc_sender,
        );

        // FIRST APPORACH: divide translation map by fixed row count. simple but inefficient
        // large agent smight be devided into dozens of submaps with no reasons
        // (each submap character count will be not reaching character limit)
        //
        /*let rows_per_map = 100;
        let mut row_counter = 0;
        for (k, v) in translation_map.drain() {
            row_counter = row_counter + 1;
            let mut maps_length = translation_maps.len();
            if row_counter > (maps_length * rows_per_map) {
                translation_maps.push(collections::HashMap::new());
                maps_length = translation_maps.len();
            }
            translation_maps[maps_length - 1].insert(k, v);
        } */

        // SECOND APPROACH: if character count represented by single map is approx. 80000 let's create new submap
        // should be fine for API quotas and this approach will result in by far smaller number of submaps
        // hence much quicker translation time
        //
        let chars_per_map = 80_000;
        let mut char_counter = 0;
        translation_maps.push(collections::HashMap::new());
        for (k, v) in translation_map.drain() {
            char_counter = char_counter + v.len();
            let mut maps_length = translation_maps.len();
            if char_counter > chars_per_map {
                translation_maps.push(collections::HashMap::new());
                maps_length = translation_maps.len();
                char_counter = 0;
            }
            translation_maps[maps_length - 1].insert(k, v);
        } /* */

        debug!("partitioned translation maps {:#?}", translation_maps);
        send_progress(
            ProgressMessageType::TextMessage("starting translation".to_owned()),
            &mpsc_sender,
        );
        send_progress(
            ProgressMessageType::CountSpecified(translation_maps.len() as u64),
            &mpsc_sender,
        );
        let mut iter_idx = 0;
        for map in translation_maps.iter() {
            iter_idx = iter_idx + 1;
            send_progress(
                ProgressMessageType::TextMessage(format!(
                    "running translation sub-batch {}/{}",
                    iter_idx,
                    translation_maps.len()
                )),
                &mpsc_sender,
            );
            let mut translated_submap = GoogleTranslateV3::execute_translation_impl(
                translated_gdf_agent_folder,
                token,
                source_lang,
                target_lang,
                project_id,
                &map,
                create_output_tsv,
                &mpsc_sender,
                iter_idx,
            )
            .await?;

            // merge translated submap into original map
            for (k, v) in translated_submap.drain() {
                translation_map.insert(k, v);
            }

            send_progress(ProgressMessageType::ItemProcessed, &mpsc_sender);
        }
        send_progress(
            ProgressMessageType::TextMessage(
                "translation finished, updating DialogFlow agent".to_owned(),
            ),
            &mpsc_sender,
        );

        debug!("applying translated map to agent");
        agent.from_translation(
            &translation_map,
            target_lang,
            skip_entities_translation,
            skip_utterances_translation,
            skip_responses_translation,
        );
        agent.add_supported_language(target_lang);
        debug!("serializing agent");
        send_progress(
            ProgressMessageType::TextMessage("storing agent to file system".to_owned()),
            &mpsc_sender,
        );
        agent.serialize(translated_gdf_agent_folder)?;
        debug!("agent serialized!");
        send_progress(
            ProgressMessageType::TextMessage("all good! exiting.".to_owned()),
            &mpsc_sender,
        );

        send_progress(ProgressMessageType::Exit, &mpsc_sender);

        Ok(())
    }

    async fn execute_translation_impl(
        translated_gdf_agent_folder: &str,
        token: &str,
        source_lang: &str,
        target_lang: &str,
        project_id: &str,
        translation_map: &collections::HashMap<String, String>,
        create_output_tsv: bool,
        mpsc_sender: &Sender<ProgressMessageType>,
        iter_idx: usize,
    ) -> Result<collections::HashMap<String, String>> {
        let ts_sec = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let storage_bucket_name_in = format!("gdf_translate_input_{}", ts_sec.to_string());
        let storage_bucket_name_out = format!("gdf_translate_output_{}", ts_sec.to_string());

        send_progress(
            ProgressMessageType::TextMessage("creating input bucket".to_owned()),
            &mpsc_sender,
        );
        let bucket_creation_result_in = storage_bucket_mgmt::create_bucket(
            token,
            project_id,
            &storage_bucket_name_in,
            "EUROPE-WEST3",
            "STANDARD",
        )
        .await?;
        debug!("bucket_creation_result_in {:#?}", bucket_creation_result_in);

        send_progress(
            ProgressMessageType::TextMessage("creating output bucket".to_owned()),
            &mpsc_sender,
        );
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

        let map_str = v3::map_to_string(translation_map);
        debug!("v3::map_to_string:\n {}", map_str);

        send_progress(
            ProgressMessageType::TextMessage("uploading translation map".to_owned()),
            &mpsc_sender,
        );
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

        send_progress(
            ProgressMessageType::TextMessage("triggering batch translation request".to_owned()),
            &mpsc_sender,
        );
        let translation_result = v3::batch_translate_text(
            token,
            project_id,
            source_lang,
            target_lang,
            "text/html", // always HTML, we are wrapping text to translate in <span> tag
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
            send_progress(
                ProgressMessageType::TextMessage("checking for translation result".to_owned()),
                &mpsc_sender,
            );
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
                    send_progress(
                        ProgressMessageType::TextMessage("batch translation completed!".to_owned()),
                        &mpsc_sender,
                    );
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
                send_progress(
                    ProgressMessageType::TextMessage("batch translation still running".to_owned()),
                    &mpsc_sender,
                );
                debug!("still running, checking the state again...")
            }
        }

        // e.g. gs://gdf_translate_output_1594998623/gdf_translate_input_1594998623_translation_map_de_translations.tsv
        let translated_object_name = format!(
            "{}_translation_map_{}_translations.tsv",
            storage_bucket_name_in, target_lang
        );

        debug!("translated_object_name {}", translated_object_name);

        send_progress(
            ProgressMessageType::TextMessage("downloading translation result".to_owned()),
            &mpsc_sender,
        );
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

        if create_output_tsv == true {
            let mut file_handle = File::create(format!(
                "{}/bucket_download_result_{}.txt",
                translated_gdf_agent_folder, iter_idx
            ))?;
            file_handle.write_all(bucket_download_result.body.as_bytes())?;
        }

        let translated_map = v3::string_to_map(bucket_download_result.body)?;

        // for deletions we are not checking API call status code
        // at this moment we have translation and do not want to interupt
        // in case we are unable to delete temporary buckets etc. we prefer
        // to leave some mess in google project and provide smooth experience for the end user
        let mut delete_object_result;

        send_progress(
            ProgressMessageType::TextMessage("deleting google cloud temporary buckets".to_owned()),
            &mpsc_sender,
        );
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

        send_progress(
            ProgressMessageType::TextMessage("returning translation map".to_owned()),
            &mpsc_sender,
        );
        debug!("translation finished. updated translation map");
        debug!("{:#?}", translated_map);

        Ok(translated_map)
    }
}

impl DummyTranslate {
    #[allow(dead_code)]
    async fn execute_translation(
        gdf_agent_path: &str,
        translated_gdf_agent_folder: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<()> {
        debug!("processing agent {}", gdf_agent_path);
        let mut agent = parse_gdf_agent_zip(gdf_agent_path)?;
        let mut translation_map =
            agent.to_translation(source_lang, target_lang, false, false, false);
        dummy_translate(&mut translation_map);
        agent.from_translation(&translation_map, target_lang, false, false, false);
        agent.add_supported_language(target_lang);
        agent.serialize(translated_gdf_agent_folder)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::google::gcloud::auth::*;
    use crate::init_logging; // set RUST_LOG=gdf_translate::google::gcloud::translate=debug
    use std::sync::mpsc::channel;

    const SAMPLE_AGENTS_FOLDER: &str =
        "C:/Users/abezecny/adam/WORK/_DEV/Rust/gdf_translate/examples/sample_agents/";

    // cargo test -- --show-output test_execute_translation_dummy
    #[test]
    #[ignore]
    fn test_execute_translation_dummy() -> Result<()> {
        init_logging();
        let agent_path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Currency-Converter.zip");

        let _ = task::block_on(DummyTranslate::execute_translation(
            &agent_path,
            "c:/tmp/out_translated",
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
        let (tx, _) = channel::<ProgressMessageType>();
        let _ = GoogleTranslateV2::execute_translation(
            &agent_path,
            "c:/tmp/out_translated",
            &token,
            "en",
            "de",
            tx,
            1,
            false,
            false,
            false,
        );

        Ok(())
    }

    // cargo test -- --show-output test_execute_translation_google_v3
    #[test]
    #[ignore]
    fn test_execute_translation_google_v3() -> Result<()> {
        init_logging();
        // let agent_path = format!("c:/tmp/Express_CS_AP_PRD.zip");
        // let agent_path = format!("c:/tmp/Currency-Converter.zip");
        let agent_path = format!("{}{}", SAMPLE_AGENTS_FOLDER, "Currency-Converter.zip");
        debug!("getting bearer token...");
        let token: Result<GoogleApisOauthToken> = task::block_on(get_google_api_token(
            "./examples/testdata/credentials_v3.json",
        ));
        let token = format!("Bearer {}", token.unwrap().access_token);
        debug!("bearer token retrieved {}", token);
        let (tx, _) = channel::<ProgressMessageType>();
        let translation_result = task::block_on(GoogleTranslateV3::execute_translation(
            &agent_path,
            "c:/tmp/out_translated",
            &token,
            "en",
            "de",
            "express-tracking",
            tx,
            false,
            false,
            false,
            false,
        ));

        debug!("translation_result: {:#?}", translation_result);

        Ok(())
    }
}

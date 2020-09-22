use crate::errors::{Error, Result};
use crate::google::dialogflow::agent::parse_gdf_agent_zip;
use crate::google::gcloud::storage_bucket_mgmt;
use crate::ui::ProgressMessageType;
use async_std::task;
use std::fs;
// while StreamExt is not used directly without it this line will not compile:
// while let Some(future_value) = futures.next().await
use crate::google::gcloud::ApiResponse;
use crate::html;
use futures::stream::{FuturesUnordered, StreamExt};
use log::debug;
use std::collections;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::time::SystemTime;
use std::{thread, time};

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

pub struct TranslationGlossary {
    pub content: String,
    pub glossary_name: String,
    pub glossary_bucket_name: String,
}

impl TranslationGlossary {
    pub fn new(glossary_name: &str) -> Self {
        let content = format!(
            "{}\n{}\n{}\n",
            "<to_translate>\t<to_translate>",
            "</to_translate>\t</to_translate>",
            "<MULTILINE />\t<MULTILINE />"
        );
        TranslationGlossary {
            content,
            glossary_name: glossary_name.to_owned(),
            glossary_bucket_name: format!("gs://{}/{}", glossary_name, glossary_name),
        }
    }

    pub fn add(&mut self, content: String) {
        self.content = format!("{}{}", self.content, content);
    }
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
        glossary_path: Option<&str>,
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

            let translation_format;
            if html::is_html(val) == true {
                translation_format = v2::TranslateFormat::Html;
            } else {
                translation_format = v2::TranslateFormat::Plain;
            }

            let mut translation_response;
            let mut translation_result =
                v2::translate(token, source_lang, target_lang, val, &translation_format).await;

            if let Err(translation_error) = translation_result {
                debug!(
                    "error while translating value {}/{} for sub-batch: {}. Attempting one more time. Error detail: {:#?}",
                    translated_item_idx, translation_count, iter_idx, translation_error
                );
                task::sleep(Duration::from_secs(2)).await; // wait with this task execution before next try!
                translation_result =
                    v2::translate(token, source_lang, target_lang, val, &translation_format).await;
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
                translation_result =
                    v2::translate(token, source_lang, target_lang, val, &translation_format).await;

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
    pub fn execute_translation(
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
        glossary_path: Option<&str>,
    ) -> Result<()> {
        debug!("processing agent {}", gdf_agent_path);

        let progress = |msg: &str| {
            send_progress(
                ProgressMessageType::TextMessage(msg.to_owned()),
                &mpsc_sender,
            );
        };

        progress("parsing zip file");
        let mut agent = parse_gdf_agent_zip(gdf_agent_path)?;

        progress("preparing translation map");
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
        progress("partitioning translation map");

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
        progress("starting translation");
        send_progress(
            ProgressMessageType::CountSpecified(translation_maps.len() as u64),
            &mpsc_sender,
        );

        let ts_millis = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let glossary_bucket_name = format!("gdf_translate_glossary_{}", ts_millis.to_string());

        progress(&format!("creating bucket {}", glossary_bucket_name));

        let bucket_creation_result_glossary = task::block_on(
            GoogleTranslateV3::create_glossary_bucket(&glossary_bucket_name, token, project_id),
        )?;

        debug!(
            "bucket {} result {:?}",
            glossary_bucket_name, bucket_creation_result_glossary
        );

        progress(&format!("bucket {} created", glossary_bucket_name));

        let mut translation_glossary = TranslationGlossary::new(&glossary_bucket_name); // glossary name will be same as the bucket name
        if let Some(glossary) = glossary_path {
            progress("loading glossary file");
            let glossary_str = fs::read_to_string(glossary)?;
            translation_glossary.add(glossary_str);
        }

        let bucket_upload_result = task::block_on(storage_bucket_mgmt::upload_object(
            token,
            &glossary_bucket_name,
            &format!("{}.tsv", &translation_glossary.glossary_name),
            &translation_glossary.content,
        ))?;

        debug!("bucket_upload_result {:#?}", bucket_upload_result);

        if bucket_upload_result.status_code != "200" {
            return Err(Error::new(format!(
                "GoogleTranslateV3.execute_translation error when uploading bucket {:#?}",
                bucket_upload_result
            )));
        }

        debug!("glossary content:\n{}", translation_glossary.content);

        progress("creating glossary");
        task::block_on(v3::create_glossary(
            token,
            project_id,
            source_lang,
            target_lang,
            &translation_glossary.glossary_name,
            &format!("{}.tsv", &translation_glossary.glossary_bucket_name),
        ))?;
        progress("glossary created");

        let mut futures = FuturesUnordered::new();

        let mut iter_idx = 0;
        while let Some(map) = translation_maps.pop() {
            iter_idx = iter_idx + 1;

            let ts_millis = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();

            // wait 2s to ensure:
            // - next iteration will get uniqueue bucket names!
            // - buckets will be created OK. It seems Google Cloud API is very sensitive when creating buckets rapidly in sequence
            thread::sleep(time::Duration::from_millis(20_000));

            let storage_bucket_name_in = format!("gdf_translate_input_{}", ts_millis.to_string());
            let storage_bucket_name_out = format!("gdf_translate_output_{}", ts_millis.to_string());

            progress(&format!("creating bucket {}", storage_bucket_name_in));
            progress(&format!("creating bucket {}", storage_bucket_name_out));

            let (bucket_creation_result_in, bucket_creation_result_out) =
                task::block_on(GoogleTranslateV3::create_translation_buckets(
                    &storage_bucket_name_in,
                    &storage_bucket_name_out,
                    token,
                    project_id,
                ))?;
            debug!(
                "bucket {} result {:?}",
                storage_bucket_name_in, bucket_creation_result_in
            );
            debug!(
                "bucket {} result {:?}",
                storage_bucket_name_out, bucket_creation_result_out
            );

            progress(&format!("bucket {} created", storage_bucket_name_in));
            progress(&format!("bucket {} created", storage_bucket_name_out));

            // keeping here if needed for debugging
            /* progress(&format!(
                "bucket {} result {:?}",
                storage_bucket_name_in, bucket_creation_result_in
            ));
            progress(&format!(
                "bucket {} result {:?}",
                storage_bucket_name_out, bucket_creation_result_out
            )); */

            let future = GoogleTranslateV3::execute_translation_impl(
                translated_gdf_agent_folder,
                token,
                source_lang,
                target_lang,
                project_id,
                map,
                create_output_tsv,
                &mpsc_sender,
                iter_idx,
                storage_bucket_name_in.to_owned(),
                storage_bucket_name_out.to_owned(),
            );
            futures.push(future);
        } // while let Some(map) = translation_maps.pop()

        // inspired by https://www.philipdaniels.com/blog/2019/async-std-demo1/
        // https://users.rust-lang.org/t/futuresunordered/39461/4
        // uses asynchronous streams (see next() method), unfortunatelly this is still not described in async-std documentation (see https://book.async.rs/concepts/streams.html)
        // so it required little bit of investigation, not sure whether this is optimal way, probably tokio has better (and better documented) capabilities when it comes to joining the futures
        task::block_on(async {
            while let Some(future_value) = futures.next().await {
                // for this to compile StreamExt must be used, see use futures::stream::{FuturesUnordered, StreamExt}; !
                match future_value {
                    Ok(translated_submap) => translation_map.extend(translated_submap),
                    Err(e) => debug!(" Error when resolving future returned by GoogleTranslateV3::execute_translation_impl : {:#?}", e),
                    // TBD: emit some text to CLI, maybe terminate the processing?
                }
            }
        });

        debug!("translation finished. updated translation map");
        debug!("{:#?}", translation_map);

        progress("deleting glossary");
        let glossary_deletion_result = task::block_on(v3::delete_glossary(
            token,
            project_id,
            &glossary_bucket_name,
        ));
        if let Err(glossary_deletion_error) = glossary_deletion_result {
            // do not terminate processing in case of failure!
            // glossary deletion is not really important from user perspective
            progress("glossary deletion failed. Delete it manually!");
            debug!("glossary deletion error {:#?}", glossary_deletion_error);
        } else {
            progress("glossary deleted");
        }

        debug!("deleting {}.tsv", &translation_glossary.glossary_name);
        let delete_object_result = task::block_on(storage_bucket_mgmt::delete_object(
            token,
            &glossary_bucket_name,
            &format!("{}.tsv", &translation_glossary.glossary_name),
        ));
        debug!(
            "delete {}.tsv result: {:#?}",
            &translation_glossary.glossary_name, delete_object_result
        );

        debug!("deleting {}", &glossary_bucket_name);
        let delete_glossary_bucket_result = task::block_on(storage_bucket_mgmt::delete_bucket(
            token,
            &glossary_bucket_name,
        ));
        if let Err(glossary_bucket_deletion_error) = delete_glossary_bucket_result {
            progress("glossary bucket deletion failed. Delete it manually!");
            debug!(
                "glossary bucket deletion error {:#?}",
                glossary_bucket_deletion_error
            );
        } else {
            debug!(
                "delete_bucket_result_out {:#?}",
                delete_glossary_bucket_result
            );
        }

        progress("translation finished, updating DialogFlow agent");

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
        progress("storing agent to file system");
        agent.serialize(translated_gdf_agent_folder)?;
        debug!("agent serialized!");
        progress("all good! exiting.");

        send_progress(ProgressMessageType::Exit, &mpsc_sender);

        Ok(())
    }

    async fn create_translation_buckets(
        storage_bucket_name_in: &str,
        storage_bucket_name_out: &str,
        token: &str,
        project_id: &str,
    ) -> Result<(ApiResponse, ApiResponse)> {
        let bucket_creation_result_in = storage_bucket_mgmt::create_bucket(
            token,
            project_id,
            &storage_bucket_name_in,
            "EUROPE-WEST3",
            "STANDARD",
        )
        .await?;
        debug!("bucket_creation_result_in {:#?}", bucket_creation_result_in);

        if bucket_creation_result_in.status_code != "200" {
            return Err(Error::new(format!(
                "GoogleTranslateV3.execute_translation error when creating bucket {:#?}",
                bucket_creation_result_in
            )));
        }

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

        Ok((bucket_creation_result_in, bucket_creation_result_out))
    }

    async fn create_glossary_bucket(
        glossary_bucket_name: &str,
        token: &str,
        project_id: &str,
    ) -> Result<ApiResponse> {
        let bucket_creation_result_glossary = storage_bucket_mgmt::create_bucket(
            token,
            project_id,
            &glossary_bucket_name,
            "EUROPE-WEST3",
            "STANDARD",
        )
        .await?;
        debug!(
            "bucket_creation_result_glossary {:#?}",
            bucket_creation_result_glossary
        );

        if bucket_creation_result_glossary.status_code != "200" {
            return Err(Error::new(format!(
                "GoogleTranslateV3.execute_translation error when creating bucket {:#?}",
                bucket_creation_result_glossary
            )));
        }

        Ok(bucket_creation_result_glossary)
    }

    async fn execute_translation_impl(
        translated_gdf_agent_folder: &str,
        token: &str,
        source_lang: &str,
        target_lang: &str,
        project_id: &str,
        translation_map: collections::HashMap<String, String>,
        create_output_tsv: bool,
        mpsc_sender: &Sender<ProgressMessageType>,
        iter_idx: usize,
        storage_bucket_name_in: String,
        storage_bucket_name_out: String,
    ) -> Result<collections::HashMap<String, String>> {
        let progress = |msg: String| {
            send_progress(ProgressMessageType::TextMessage(msg), &mpsc_sender);
        };

        let map_str = v3::map_to_string(&translation_map);
        debug!("v3::map_to_string:\n {}", map_str);

        progress(format!("uploading translation map {}", iter_idx));
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

        progress(format!("triggering batch translation request {}", iter_idx));
        let translation_result = v3::batch_translate_text(
            token,
            project_id,
            source_lang,
            target_lang,
            "text/html", // always HTML, we are wrapping text to translate in <span> tag
            &format!("gs://{}/translation_map.tsv", storage_bucket_name_in),
            &format!("gs://{}/", storage_bucket_name_out),
            Some("glossary-en-sv"),
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
            // progress("checking for translation result");
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
                    progress(format!("batch translation completed {}", iter_idx));
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
                progress(format!("batch translation still running {}", iter_idx));
                debug!("still running, checking the state again...")
            }
        }

        // e.g. gs://gdf_translate_output_1594998623/gdf_translate_input_1594998623_translation_map_de_translations.tsv
        let translated_object_name = format!(
            "{}_translation_map_{}_translations.tsv",
            storage_bucket_name_in, target_lang
        );

        debug!("translated_object_name {}", translated_object_name);

        progress(format!("downloading translation result {}", iter_idx));
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

        // keep user updated asap, deletion is not that important.
        // if api returns non-200 sattus code we are ignoring it anyway
        send_progress(ProgressMessageType::ItemProcessed, &mpsc_sender);

        // for deletions we are not checking API call status code
        // at this moment we have translation and do not want to interupt
        // in case we are unable to delete temporary buckets etc. we prefer
        // to leave some mess in google project and provide smooth experience for the end user
        let mut delete_object_result;

        progress(format!(
            "deleting google cloud temporary buckets {}",
            iter_idx
        ));
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

        // progress(format!("returning translation map {}", iter_idx));
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
            None,
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
        let translation_result = GoogleTranslateV3::execute_translation(
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
            None,
        );

        debug!("translation_result: {:#?}", translation_result);

        Ok(())
    }
}

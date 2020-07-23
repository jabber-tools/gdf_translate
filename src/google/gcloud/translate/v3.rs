//! # implementation of google translation api v3
//!
//! See following links
//!
//! * [Google Translate V3 Intro](https://cloud.google.com/translate/docs/intro-to-v3)
//! * [translateText API](https://cloud.google.com/translate/docs/reference/rest/v3/projects/translateText)
//! * [batchTranslateText API](https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations/batchTranslateText)
//! * [batch translation result -long running operation](https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations.operations#Operation)
//! * [get long running operatopm result - short polling approach](https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations.operations/get)
//! * [get long running operatopm result - long polling approach](https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations.operations/wait)
//!
//! Sample curls:
//!
//!
//! Initiate batch translation
//! ```ignore
//! curl --location --request POST &apos;https://translation.googleapis.com/v3/projects/dummy-project-id/locations/us-central1:batchTranslateText&apos; \
//! --header &apos;Authorization: Bearer ya29.c....&apos; \
//! --header &apos;Content-Type: application/javascript&apos; \
//! --data-raw &apos;{
//!     "sourceLanguageCode": "en",
//!     "targetLanguageCodes": "de",
//!     "inputConfigs": [{
//!         "mimeType":  "text/html",
//!         "gcsSource": {
//!             "inputUri": "gs://translate_v3_test_in/input.tsv"
//!         }
//!     }],
//!     "outputConfig": {
//!         "gcsDestination": {
//!             "outputUriPrefix": "gs://translate_v3_test_out/"
//!         }
//!     }
//! }&apos;
//! ```
//!
//!
//! Check long running operation status with immediate response (kind of short polling)
//! ```ignore
//! curl --location --request GET &apos;https://translation.googleapis.com/v3/projects/dummy-project-id/locations/us-central1/operations/20200615-11581592247524-5edeccd9-0000-26b7-bd4f-30fd38139c64&apos; \
//! --header &apos;Authorization: Bearer ya29.c....&apos; \
//! ```
//!
//!
//! Check long running operation status with delayed response (kind of long polling)
//! ```ignore
//! curl --location --request POST &apos;https://translation.googleapis.com/v3/projects/dummy-project-id/locations/us-central1/operations/20200615-11581592247524-5edeccd9-0000-26b7-bd4f-30fd38139c64:wait&apos; \
//! --header &apos;Authorization: Bearer ya29.c....&apos; \
//! --header &apos;Content-Type: application/json&apos; \
//! --data-raw &apos;{
//!   "timeout": "60s"
//! }&apos;
//! ```
//!
use crate::errors::Result;
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections;

lazy_static! {
    // represents any word with 0..n white characters at the end terminated by triple semicolon
    // specific case is ' ;' i.e. jsut white spaces with colon without any leading letters/digits (hence \w* and not \w+)
    pub static ref RE_WORD_WITH_SPACES: Regex = Regex::new(r"span\w*(\s*)span").unwrap();
}

fn get_trailing_spaces(s: &str) -> String {
    let caps_opt = RE_WORD_WITH_SPACES.captures(s);
    if let Some(caps) = caps_opt {
        let trailing_spaces = caps[1].to_owned();
        return trailing_spaces;
    } else {
        return "".to_owned();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3ResponseMetadata {
    #[serde(rename = "@type")]
    pub type_attr: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3Response {
    pub name: String,
    pub metadata: GoogleTranslateV3ResponseMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3ApiResponse {
    pub status_code: String,
    pub body: GoogleTranslateV3Response,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3WaitResponseMetadata {
    #[serde(rename = "@type")]
    pub type_attr: String,
    pub state: String,

    #[serde(rename = "submitTime")]
    pub submit_time: String,

    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3WaitResponseMeResponse {
    #[serde(rename = "@type")]
    pub type_attr: String,

    #[serde(rename = "totalCharacters")]
    pub total_characters: String,

    #[serde(rename = "translatedCharacters")]
    pub translated_characters: String,

    #[serde(rename = "submitTime")]
    pub submit_time: String,

    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3WaitResponseErrorDetail {
    #[serde(rename = "@type")]
    pub type_attr: String,

    pub detail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3WaitResponseError {
    pub code: u32,
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<GoogleTranslateV3WaitResponseErrorDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3WaitResponse {
    pub name: String,
    pub metadata: GoogleTranslateV3WaitResponseMetadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<GoogleTranslateV3WaitResponseError>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<GoogleTranslateV3WaitResponseMeResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3WaitApiResponse {
    pub status_code: String,
    pub body: GoogleTranslateV3WaitResponse,
}

pub fn map_to_string(translation_map: &collections::HashMap<String, String>) -> String {
    let mut s = String::from("");

    for (key, val) in translation_map.iter() {
        // ;;; is just delimiter that is highly unprobable to occur in real intent text (while ; can still be there)
        // we are adding semicolon to preserve information about spaces at the end of oroginal string!
        s.push_str(&format!("{} ;;;{};;;\n", key, val));
    }

    s
}

/// Converts string produced by Google Translate V3 API back to translation map
///
///
///
/// Arguments:
/// * `s`: String of translation map as produced by Google Translate V3 API. Example:
/// ```ignore
/// 0000000000	7f06092ac6d0 ;;;translate me;;;	7f06092ac6d0 ;;;übersetze mich;;;
/// 0000000001	7f06092ac6d1 ;;;rust is great;;;	7f06092ac6d1 ;;;Rost ist großartig;;;
/// 0000000002	7f06092ac6d2 ;;;let's have a weekend;;;	7f06092ac6d2 ;;;Lass uns ein Wochenende haben;;;
/// ```
///
/// Returns: input above should return following map:
/// ```ignore
/// KEY                 VAL
/// -------------------------------------------------
/// 7f06092ac6d0        übersetze mich
/// 7f06092ac6d1        Rost ist großartig
/// 7f06092ac6d2        Lass uns ein Wochenende haben
/// ```
/// In this map KEY represents address of rust structure field reference
/// and value represents translaetd text to be applied
///
pub fn string_to_map(s: String) -> collections::HashMap<String, String> {
    let mut translation_map: collections::HashMap<String, String> = collections::HashMap::new();
    let split = s.split("\n");

    let vec: Vec<&str> = split.collect();

    for item in vec.iter() {
        if item.trim() == "" {
            continue; // skip the last empty row
        }

        let mut white_space_iter = item.split_whitespace();
        white_space_iter.next(); // get 0000000000
        let address = white_space_iter.next().unwrap(); // get 7f06092ac6d0

        // this will capture original text between addresses i.e. From:
        // 7f06092ac6d0 ;;;translate me;;;	7f06092ac6d0
        // it will capture
        //  ;;;translate me;;;
        // using dynamically compiled regex is performance inefficient but there is no way how to avoid it here
        // ;;; is just delimiter that is highly unprobable to occur in real intent text (while ; can still be there)
        let regex_orig_text =
            Regex::new(&format!(r"{}\s+(;;;.*;;;)\s+{}", address, address)).unwrap();
        println!(">>>>processing item: {}", item);
        let orig_text_captures = regex_orig_text.captures(item).unwrap();
        let orig_text = orig_text_captures[1].to_owned();
        let orig_text = orig_text.trim_end();
        //  we want to capture trailing spaces so that we can add them to translated text!
        // we need to do it since google v3 translate api will not preserve trailing spaces in tsv format!
        let orig_text_trailing_spaces = get_trailing_spaces(&orig_text);

        let idx = item.rfind(address);

        let idx = idx.unwrap() + address.len();

        let translation = item[idx..item.len()].trim().replace(";;;", "");
        println!("translation1:{}<<orig_text_trailing_spaces:{}<<<",translation,orig_text_trailing_spaces);
        let new_text = format!(
            "{}{}",
            translation,
            orig_text_trailing_spaces
        );
        translation_map.insert(address.to_owned(), new_text);
    }

    translation_map
}

/// Translates csv/tsv file using Google Translate V3 REST API
///
/// See: https://cloud.google.com/translate/docs/reference/rest/v3/projects/translateText
///
/// Arguments:
///
/// * `token`: Bearer token
/// * `project_id`: Google project ID
/// * `source_lang`: e.g. 'en'
/// * `target_lang`: e.g. 'de'
/// * `mime_type`: text/html or text/plain
/// * `input_uri`: e.g. gs://translate_v3_test_in/input.tsv
/// * `output_uri_prefix`: e.g. gs://translate_v3_test_out/
pub async fn batch_translate_text(
    token: &str,
    project_id: &str,
    source_lang: &str,
    target_lang: &str,
    mime_type: &str,
    input_uri: &str,
    output_uri_prefix: &str,
) -> Result<GoogleTranslateV3ApiResponse> {
    let url = format!(
        "https://translation.googleapis.com/v3/projects/{}/locations/us-central1:batchTranslateText",
        project_id
    );

    let body = json!({
        "sourceLanguageCode": source_lang,
        "targetLanguageCodes": target_lang,
        "inputConfigs": [{
            "mimeType":  mime_type,
            "gcsSource": {
                "inputUri": input_uri
            }
        }],
        "outputConfig": {
            "gcsDestination": {
                "outputUriPrefix": output_uri_prefix
            }
        }
    });

    debug!("body: {}", body);
    debug!("url: {}", url);

    let mut resp = surf::post(url)
        .set_header("Authorization", token)
        .body_json(&body)?
        .await?;

    let response_body: GoogleTranslateV3Response =
        serde_json::from_str(&resp.body_string().await?)?;

    Ok(GoogleTranslateV3ApiResponse {
        status_code: resp.status().as_str().to_string(),
        body: response_body,
    })
}

/// Check the status of long running operation representing batch translation request
///
/// * `token`: Bearer token
/// * `long_running_operation`: something like projects/345634260051/locations/us-central1/operations/20200711-05421594471378-5f058a16-0000-2dd4-8106-883d24f67490.
/// Returned by https://translation.googleapis.com/v3/projects/{}/locations/us-central1:batchTranslateText API
pub async fn batch_translate_text_check_status(
    token: &str,
    long_running_operation: &str,
) -> Result<GoogleTranslateV3WaitApiResponse> {
    let url = format!(
        "https://translation.googleapis.com/v3/{}:wait",
        long_running_operation
    );

    let body = json!({
        "timeout": "60s"
    });

    debug!("url: {}", url);
    debug!("body: {}", body);

    let mut resp = surf::post(url)
        .set_header("Authorization", token)
        .body_json(&body)?
        .await?;

    let body_str = resp.body_string().await?;

    debug!("response body: {}", body_str);

    let response_body: GoogleTranslateV3WaitResponse = serde_json::from_str(&body_str)?;

    Ok(GoogleTranslateV3WaitApiResponse {
        status_code: resp.status().as_str().to_string(),
        body: response_body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::google::gcloud::auth::*;
    use crate::init_logging; // set RUST_LOG=gdf_translate::google::gcloud::translate::v3=debug
    use async_std::task;

    // cargo test -- --show-output test_batch_translate_text
    #[test]
    //#[ignore]
    fn test_batch_translate_text() -> Result<()> {
        init_logging();
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);

        println!("access_token {:#?}", token);
        let api_response: Result<GoogleTranslateV3ApiResponse> =
            task::block_on(batch_translate_text(
                &token,
                "express-tracking",
                "en",
                "de",
                "text/html",
                "gs://translate_v3_test/translation_map.tsv",
                "gs://translate_v3_test_out/",
            ));

        let api_response = api_response.unwrap();
        println!("api_response {:#?}", api_response);

        let api_response2: Result<GoogleTranslateV3WaitApiResponse> = task::block_on(
            batch_translate_text_check_status(&token, &api_response.body.name),
        );

        println!("api_response2 {:#?}", api_response2);
        Ok(())
    }

    // cargo test -- --show-output test_batch_translate_text_wait
    #[test]
    #[ignore]
    fn test_batch_translate_text_wait() -> Result<()> {
        init_logging();
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);

        println!("access_token {:#?}", token);
        let api_response: Result<GoogleTranslateV3WaitApiResponse> = task::block_on(
            batch_translate_text_check_status(
                &token,
                "projects/345634260051/locations/us-central1/operations/20200711-06301594474232-5f0599bc-0000-2328-9a34-883d24f6d7a8"
            ),
        );

        println!("api_response {:#?}", api_response);
        Ok(())
    }

    // cargo test -- --show-output test_deser_google_translate_v3_wait_response
    #[test]
    fn test_deser_google_translate_v3_wait_response() -> Result<()> {
        let response_running = r#"
        {
            "name": "projects/345634260051/locations/us-central1/operations/20200711-05411594471274-5f058a7e-0000-2140-8a4b-24058878f154",
            "metadata": {
              "@type": "type.googleapis.com/google.cloud.translation.v3.BatchTranslateMetadata",
              "state": "RUNNING",
              "totalCharacters": "52",
              "submitTime": "2020-07-11T12:41:14Z"
            }
          }
        "#;

        let response_done = r#"
        {
            "name": "projects/345634260051/locations/us-central1/operations/20200711-05411594471274-5f058a7e-0000-2140-8a4b-24058878f154",
            "metadata": {
              "@type": "type.googleapis.com/google.cloud.translation.v3.BatchTranslateMetadata",
              "state": "SUCCEEDED",
              "translatedCharacters": "52",
              "totalCharacters": "52",
              "submitTime": "2020-07-11T12:41:14Z"
            },
            "done": true,
            "response": {
              "@type": "type.googleapis.com/google.cloud.translation.v3.BatchTranslateResponse",
              "totalCharacters": "52",
              "translatedCharacters": "52",
              "submitTime": "2020-07-11T12:41:14Z",
              "endTime": "2020-07-11T12:42:23Z"
            }
          }
        "#;

        let response_failed = r#"
        {
            "name": "projects/345634260051/locations/us-central1/operations/20200711-05421594471378-5f058a16-0000-2dd4-8106-883d24f67490",
            "metadata": {
              "@type": "type.googleapis.com/google.cloud.translation.v3.BatchTranslateMetadata",
              "state": "FAILED",
              "submitTime": "2020-07-11T12:42:58Z"
            },
            "done": true,
            "error": {
              "code": 3,
              "message": "Output uri prefix must be an empty bucket",
              "details": [
                {
                  "@type": "type.googleapis.com/google.rpc.DebugInfo",
                  "detail": " cloud/ml/api/translation/service/v3/orchestration_batch_server/batch_translation_job_handler.cc:2516. project_number: 345634260051, job_id: 20200711-05421594471378-5f058a16-0000-2dd4-8106-883d24f67490. "
                },
                {
                  "@type": "type.googleapis.com/google.rpc.DebugInfo",
                  "detail": " cloud/ml/api/translation/service/v3/orchestration_batch_server/batch_translation_job_handler.cc:373. project_number: 345634260051, job_id: 20200711-05421594471378-5f058a16-0000-2dd4-8106-883d24f67490. "
                }
              ]
            }
          }
        "#;

        let response_body: GoogleTranslateV3WaitResponse = serde_json::from_str(&response_running)?;
        assert_eq!(response_body.name, "projects/345634260051/locations/us-central1/operations/20200711-05411594471274-5f058a7e-0000-2140-8a4b-24058878f154");

        let response_body: GoogleTranslateV3WaitResponse = serde_json::from_str(&response_done)?;
        if let Some(done) = response_body.done {
            assert_eq!(done, true);
        } else {
            assert_eq!(false, true);
        }

        let response_body: GoogleTranslateV3WaitResponse = serde_json::from_str(&response_failed)?;

        if let Some(done) = response_body.done {
            assert_eq!(done, true);
        } else {
            assert_eq!(false, true);
        }

        if let Some(error) = response_body.error {
            assert_eq!(error.code, 3);
            assert_eq!(error.message, "Output uri prefix must be an empty bucket");
        } else {
            assert_eq!(false, true);
        }

        Ok(())
    }

    // cargo test -- --show-output test_string_to_map_1
    #[test]
    fn test_string_to_map_1() -> Result<()> {
        let translated_map_str = r#"
        0000000000	7f06092ac6d0 ;;;translate me;;;	7f06092ac6d0 ;;;übersetze mich;;;
        0000000001	7f06092ac6d1 ;;;rust is great ;;;	7f06092ac6d1 ;;;Rost ist großartig;;;
        0000000002	7f06092ac6d2 ;;;let's have a weekend;;;	7f06092ac6d2 ;;;Lass uns ein Wochenende haben;;;
        0000000003	7f06092ac6d3 ;;; ;;;	7f06092ac6d3  ;;; ;;;
        "#;

        let translated_map = string_to_map(translated_map_str.to_string());
        assert_eq!(translated_map.len(), 4);

        println!("translated_map: {:#?}", translated_map);

        assert_eq!(
            translated_map.get("7f06092ac6d0").unwrap(),
            "übersetze mich"
        );
        assert_eq!(
            translated_map.get("7f06092ac6d1").unwrap(),
            "Rost ist großartig "
        );
        assert_eq!(
            translated_map.get("7f06092ac6d2").unwrap(),
            "Lass uns ein Wochenende haben"
        );
        assert_eq!(translated_map.get("7f06092ac6d3").unwrap(), " ");
        Ok(())
    }

    // cargo test -- --show-output test_string_to_map_2
    #[test]
    fn test_string_to_map_2() -> Result<()> {
        let translated_map_str = r#"
        0000000000	7f06092ac6d0 ;;;translate me;;;	7f06092ac6d0 ;;;翻譯我;;;
        0000000001	7f06092ac6d1 ;;;rust is great;;;	7f06092ac6d1 ;;;銹很棒;;;
        0000000002	7f06092ac6d2 ;;;let's have a weekend;;;	7f06092ac6d2 ;;;讓我們週末;;;
        "#;

        let translated_map = string_to_map(translated_map_str.to_string());
        assert_eq!(translated_map.len(), 3);

        println!("translated_map: {:#?}", translated_map);

        assert_eq!(translated_map.get("7f06092ac6d0").unwrap(), "翻譯我");
        assert_eq!(translated_map.get("7f06092ac6d1").unwrap(), "銹很棒");
        assert_eq!(translated_map.get("7f06092ac6d2").unwrap(), "讓我們週末");
        Ok(())
    }

    // cargo test -- --show-output test_string_to_map_3
    #[test]
    fn test_string_to_map_3() -> Result<()> {
        let translated_map_str = r#"
        0000000000	7f06092ac6d0 ;;;переведите меня;;;	7f06092ac6d0 ;;;ترجمة لي;;;
        0000000001	7f06092ac6d1 ;;;ржавчина это здорово;;;	7f06092ac6d1 ;;;الصدأ رائع;;;
        0000000002	7f06092ac6d2 ;;;давай проведем выходные;;;	7f06092ac6d2 ;;;لنحصل على عطلة نهاية أسبوع;;;
        "#;

        let translated_map = string_to_map(translated_map_str.to_string());
        assert_eq!(translated_map.len(), 3);

        println!("translated_map: {:#?}", translated_map);

        assert_eq!(translated_map.get("7f06092ac6d0").unwrap(), "ترجمة لي");
        assert_eq!(translated_map.get("7f06092ac6d1").unwrap(), "الصدأ رائع");
        assert_eq!(
            translated_map.get("7f06092ac6d2").unwrap(),
            "لنحصل على عطلة نهاية أسبوع"
        );
        Ok(())
    }

    // cargo test -- --show-output test_get_trailing_spaces
    #[test]
    fn test_get_trailing_spaces() {
        assert_eq!(get_trailing_spaces("spansome text   span"), "   ".to_owned());
        assert_eq!(get_trailing_spaces("span1234span"), "".to_owned());
    }

    // cargo test -- --show-output test_capturing_orig_text_1
    #[test]
    fn test_capturing_orig_text_1() {
        let item = "0000000000	7f06092ac6d0 ;;;translate me ;;;	7f06092ac6d0 ;;;übersetze mich;;;";
        let address = "7f06092ac6d0";

        let regex_orig_text =
            Regex::new(&format!(r"{}\s+(;;;.*;;;)\s+{}", address, address)).unwrap();

        let orig_text_captures = regex_orig_text.captures(item).unwrap();
        let orig_text = orig_text_captures[1].to_owned();
        let orig_text = orig_text.trim_end();
        println!("orig_text:{}<<<<", orig_text);
        assert_eq!(orig_text, ";;;translate me ;;;");
    }

    // cargo test -- --show-output test_capturing_orig_text_2
    #[test]
    fn test_capturing_orig_text_2() {
        //let item = "0000000000	7f06092ac6d0 ;;; ;;;	7f06092ac6d0 ;;;übersetze mich;;;";
        let item = "0000000003	7f06092ac6d0 ;;; ;;;	7f06092ac6d0  ;;;übersetze mich;;;";
        let address = "7f06092ac6d0";

        let regex_orig_text =
            Regex::new(&format!(r"{}\s+(;;;.*;;;)\s+{}", address, address)).unwrap();

        let orig_text_captures = regex_orig_text.captures(item).unwrap();
        let orig_text = orig_text_captures[1].to_owned();
        let orig_text = orig_text.trim_end();
        println!("orig_text:{}<<<<", orig_text);
        assert_eq!(orig_text, ";;; ;;;");
    }
}

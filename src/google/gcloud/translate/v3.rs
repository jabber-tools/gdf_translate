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
use crate::errors::{Error, Result};
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections;

lazy_static! {
    pub static ref RE_TO_TRANSLATE_CONTENT: Regex = Regex::new(r"<to_translate>(.*)</to_translate>").unwrap();

    // represents any word with 0..n white characters at the end terminated by triple semicolon
    // specific case is ' ;' i.e. jsut white spaces with colon without any leading letters/digits (hence \w* and not \w+)
    pub static ref RE_WORD_WITH_TRAILING_WHITE_CHARS: Regex = Regex::new(r"\w*(\s*)</to_translate>").unwrap();
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
pub struct GoogleTranslateV3WaitResponseResponse {
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
    pub response: Option<GoogleTranslateV3WaitResponseResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3WaitApiResponse {
    pub status_code: String,
    pub body: GoogleTranslateV3WaitResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleCreateGlossaryWaitResponseResponseInputConfig {
    #[serde(rename = "gcsSource")]
    pub gcs_source: GoogleCreateGlossaryWaitResponseResponseInputConfigGcsSource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleCreateGlossaryWaitResponseResponseInputConfigGcsSource {
    #[serde(rename = "inputUri")]
    pub input_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleCreateGlossaryWaitResponseResponseLanguagePair {
    #[serde(rename = "sourceLanguageCode")]
    pub source_language_code: String,
    #[serde(rename = "targetLanguageCode")]
    pub target_language_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleCreateGlossaryWaitResponseResponse {
    #[serde(rename = "@type")]
    pub type_attr: String,

    pub name: String,

    #[serde(rename = "languagePair")]
    pub language_pair: GoogleCreateGlossaryWaitResponseResponseLanguagePair,

    #[serde(rename = "inputConfig")]
    pub input_config: GoogleCreateGlossaryWaitResponseResponseInputConfig,

    #[serde(rename = "entryCount")]
    pub entry_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleCreateGlossaryWaitResponse {
    pub name: String,
    pub metadata: GoogleTranslateV3WaitResponseMetadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,

    // not really tested whether we can get this and in this structure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<GoogleTranslateV3WaitResponseError>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<GoogleCreateGlossaryWaitResponseResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleCreateGlossaryWaitApiResponse {
    pub status_code: String,
    pub body: GoogleCreateGlossaryWaitResponse,
}

/// represents line fom output file of google translate v3 batch api
/// Something like this:
/// 0x16f8c9bd8b0 <to_translate>translate me</to_translate>	<to_translate> übersetze mich </to_translate>
#[derive(Debug, PartialEq)]
struct TsvLine {
    ref_addr: String,
    orig_text: String,
    translated_text: String,
}

/// converts translation hashmap into rows with following structure
/// 0x16f8c9bd8b0 <to_translate>rust is great</to_translate>
/// i.e. address of respective reference + its content wrapped in <to_translate></to_translate>  tags
/// address must be separated from actual content by \t since it is TSV file!
pub fn map_to_string(translation_map: &collections::HashMap<String, String>) -> String {
    let mut s = String::from("");

    for (key, val) in translation_map.iter() {
        // value to translate will be wrapped in <to_translate> tag to preserve
        // all leading and trailing white characters from original value
        s.push_str(&format!(
            "{}\t<to_translate>{}</to_translate>\n",
            key,
            val.replace("\r\n", "<MULTILINE />") // tsv format does not allow mutlilines! => \n / \r\n -> <MULTILINE />
                .replace("\n", "<MULTILINE />") // once we retrieve translation from google we will replace <MULTILINE /> -> \n in parse_tsv_line
                .replace("\"", "&quot;"),
        ));
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
/// 0x16f8c9bd8b1 <to_translate>translate me</to_translate>	<to_translate> übersetze mich </to_translate>
/// 0x16f8c9bd8b2 <to_translate>rust is great</to_translate>	<to_translate> Rost ist großartig </to_translate>
/// 0x16f8c9bd8b3 <to_translate>let's have a weekend</to_translate>	<to_translate> Lass uns ein Wochenende haben </to_translate>
/// ```
///
/// Returns: input above should return following map:
/// ```ignore
/// KEY                 VAL
/// -------------------------------------------------
/// 0x16f8c9bd8b1        übersetze mich
/// 0x16f8c9bd8b2        Rost ist großartig
/// 0x16f8c9bd8b3        Lass uns ein Wochenende haben
/// ```
/// In this map KEY represents address of rust structure field reference
/// and value represents translated text to be applied
///
pub fn string_to_map(s: String) -> Result<collections::HashMap<String, String>> {
    let mut translation_map: collections::HashMap<String, String> = collections::HashMap::new();
    let split = s.split("\n");

    let vec: Vec<&str> = split.collect();

    for item in vec.iter() {
        if item.trim() == "" {
            continue; // skip the last empty row
        }
        debug!("string_to_map processing item:{}<<<", item);

        let parsed_line = parse_tsv_line(&item.replace("&quot;", "\""))?;
        let mut leading_space = "";
        let mut trailing_space = "";

        if parsed_line.orig_text.starts_with(" ") == true {
            debug!("adding leading space");
            leading_space = " ";
        }

        if parsed_line.orig_text.ends_with(" ") == true && parsed_line.orig_text.len() > 1 {
            debug!("adding trailing space");
            trailing_space = " ";
        }

        let new_text = format!(
            "{}{}{}",
            leading_space, parsed_line.translated_text, trailing_space
        );

        translation_map.insert(parsed_line.ref_addr.to_owned(), new_text);
    }

    Ok(translation_map)
}

/// Parse string line like this:
/// 0x16f8c9bd8b1 <to_translate>translate me</to_translate>	<to_translate> übersetze mich </to_translate>
/// into structure TsvLine, i.e.
///  TsvLine {
///     ref_addr: "0x16f8c9bd8b1",
///     orig_text: "translate me",
///     translated_text: " übersetze mich ",
/// }
/// Leading and trailing spaces from translated_text are removed by trim
/// caller of parse_tsv_line (i.e. string_to_map) will reconstruct any potential
/// leading and trailing spaces from original text
fn parse_tsv_line(line: &str) -> Result<TsvLine> {
    debug!("parse_tsv_line processing item:{}<<<", line);
    let mut white_space_iter = line.split_whitespace();

    let address; // get 0x1bb39b97460
    if let Some(address_val) = white_space_iter.next() {
        address = address_val;
    } else {
        return Err(Error::new("Cannot retrieve address_val".to_owned()));
    }

    let regex_str = format!(r"^\s*{}\s+(<to_translate>.*</to_translate>)\s+", address);

    debug!("regex_str:{}", regex_str);

    let regex_orig_text = Regex::new(&regex_str).unwrap();

    let text_captures = regex_orig_text.captures(line).unwrap();
    debug!("text_captures:{:#?}", text_captures);

    let orig_text = text_captures[1].to_owned();
    let orig_text = RE_TO_TRANSLATE_CONTENT.captures(&orig_text).unwrap()[1].to_owned();
    debug!("orig_text:{}<<<", orig_text);

    let idx = line.rfind("<to_translate>");
    let idx = idx.unwrap();
    let translated_text = line[idx..line.len()].to_owned();
    debug!("translated_text:{}<<<", translated_text);

    // remote surrounding span tag from translated text
    let translated_text = RE_TO_TRANSLATE_CONTENT.captures(&translated_text).unwrap()[1]
        .trim()
        .to_owned();
    debug!("translated_text without span:{}<<<", translated_text);

    Ok(TsvLine {
        ref_addr: address.to_owned(),
        orig_text: orig_text.to_owned().replace("<MULTILINE />", "\n"),
        translated_text: translated_text.replace("<MULTILINE />", "\n"),
    })
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
/// * `glossary`: name of the glossary, same as glossary bucket name
pub async fn batch_translate_text(
    token: &str,
    project_id: &str,
    source_lang: &str,
    target_lang: &str,
    mime_type: &str,
    input_uri: &str,
    output_uri_prefix: &str,
    glossary: Option<&str>,
) -> Result<GoogleTranslateV3ApiResponse> {
    let url = format!(
        "https://translation.googleapis.com/v3/projects/{}/locations/us-central1:batchTranslateText",
        project_id
    );

    let body;
    if let Some(glossary_name) = glossary {
        body = json!({
            "sourceLanguageCode": source_lang,
            "targetLanguageCodes": target_lang,
            "inputConfigs": [{
                "mimeType":  mime_type,
                "gcsSource": {
                    "inputUri": input_uri
                }
            }],
            "glossaries": {
                target_lang: {
                    "glossary": format!("projects/{}/locations/us-central1/glossaries/{}", project_id, glossary_name),
                    "ignoreCase": true
                }
            },
            "outputConfig": {
                "gcsDestination": {
                    "outputUriPrefix": output_uri_prefix
                }
            }
        });
    } else {
        body = json!({
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
    }

    debug!("body: {}", body);
    debug!("url: {}", url);

    let mut resp = surf::post(url)
        .set_header("Authorization", token)
        .body_json(&body)?
        .await?;

    let body_str = resp.body_string().await?;
    debug!("batch_translate_text.body_str: {}", body_str);

    let response_body: GoogleTranslateV3Response = serde_json::from_str(&body_str)?;

    debug!("batch_translate_text.response_body: {:#?}", response_body);

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

pub async fn create_glossary(
    token: &str,
    project_id: &str,
    source_lang: &str,
    target_lang: &str,
    glossary_id: &str,
    bucket_name: &str,
) -> Result<GoogleTranslateV3Response> {
    let url = format!(
        "https://translation.googleapis.com/v3/projects/{}/locations/us-central1/glossaries",
        project_id
    );

    let glossary_name = format!(
        "projects/{}/locations/us-central1/glossaries/{}",
        project_id, glossary_id
    );
    let body = json!({
      "name": glossary_name,
      "languagePair": {
        "sourceLanguageCode": source_lang,
        "targetLanguageCode": target_lang
        },
      "inputConfig": {
        "gcsSource": {
          "inputUri": bucket_name
        }
      }
    });

    debug!("body: {}", body);
    debug!("url: {}", url);

    let mut resp = surf::post(url)
        .set_header("Authorization", token)
        .body_json(&body)?
        .await?;

    let body_str = resp.body_string().await?;
    debug!("create_glossary.body_str: {}", body_str);

    let response_body: GoogleTranslateV3Response = serde_json::from_str(&body_str)?;

    debug!("create_glossary.response_body: {:#?}", response_body);

    Ok(response_body)
}

pub async fn create_glossary_check_status(
    token: &str,
    long_running_operation: &str,
) -> Result<GoogleCreateGlossaryWaitApiResponse> {
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

    let response_body: GoogleCreateGlossaryWaitResponse = serde_json::from_str(&body_str)?;

    Ok(GoogleCreateGlossaryWaitApiResponse {
        status_code: resp.status().as_str().to_string(),
        body: response_body,
    })
}

pub async fn delete_glossary(
    token: &str,
    project_id: &str,
    glossary_id: &str,
) -> Result<GoogleTranslateV3Response> {
    let url = format!(
        "https://translation.googleapis.com/v3/projects/{}/locations/us-central1/glossaries/{}",
        project_id, glossary_id
    );

    debug!("url: {}", url);

    let mut resp = surf::delete(url).set_header("Authorization", token).await?;

    let body_str = resp.body_string().await?;
    debug!("delete_glossary.body_str: {}", body_str);

    let response_body: GoogleTranslateV3Response = serde_json::from_str(&body_str)?;

    debug!("delete_glossary.response_body: {:#?}", response_body);

    Ok(response_body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::google::gcloud::auth::*;
    use crate::init_logging; // set RUST_LOG=gdf_translate::google::gcloud::translate::v3=debug
    use async_std::task;
    use std::fs::File;
    use std::io::Write;

    // cargo test -- --show-output test_batch_translate_text
    #[test]
    #[ignore]
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
                None,
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

    // cargo test -- --show-output test_create_glossary
    #[test]
    //#[ignore]
    fn test_create_glossary() -> Result<()> {
        init_logging();
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);

        println!("access_token {:#?}", token);
        let api_response: Result<GoogleTranslateV3Response> = task::block_on(create_glossary(
            &token,
            "express-tracking",
            "en",
            "sv",
            "glossary-en-sv",
            "gs://translate_v3_test/my-glossary.tsv",
        ));

        let api_response = api_response.unwrap();
        println!("api_response {:#?}", api_response);

        let api_response2: Result<GoogleCreateGlossaryWaitApiResponse> =
            task::block_on(create_glossary_check_status(&token, &api_response.name));

        println!("api_response2 {:#?}", api_response2);

        Ok(())
    }

    // cargo test -- --show-output test_delete_glossary
    #[test]
    //#[ignore]
    fn test_delete_glossary() -> Result<()> {
        init_logging();
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);

        println!("access_token {:#?}", token);
        let api_response: Result<GoogleTranslateV3Response> =
            task::block_on(delete_glossary(&token, "express-tracking", "my-glossary"));

        let api_response = api_response.unwrap();
        println!("api_response {:#?}", api_response);

        Ok(())
    }

    // cargo test -- --show-output test_string_to_map_1
    #[test]
    fn test_string_to_map_1() -> Result<()> {
        init_logging();
        let translated_map_str = r#"
        0x16f8c9bd8b0 <to_translate>translate me</to_translate>      <to_translate>übersetze mich</to_translate>
        0x16f8c9bd8b1 <to_translate>rust is great </to_translate>      <to_translate>Rost ist großartig </to_translate>
        0x16f8c9bd8b2 <to_translate>let's have a weekend</to_translate>      <to_translate>Lass uns ein Wochenende haben</to_translate>
        0x16f8c9bd8b3 <to_translate>   </to_translate>	      <to_translate>   </to_translate>
        "#;

        let translated_map = string_to_map(translated_map_str.to_string())?;
        assert_eq!(translated_map.len(), 4);

        println!("translated_map: {:#?}", translated_map);

        assert_eq!(
            translated_map.get("0x16f8c9bd8b0").unwrap(),
            "übersetze mich"
        );
        assert_eq!(
            translated_map.get("0x16f8c9bd8b1").unwrap(),
            "Rost ist großartig "
        );
        assert_eq!(
            translated_map.get("0x16f8c9bd8b2").unwrap(),
            "Lass uns ein Wochenende haben"
        );
        assert_eq!(translated_map.get("0x16f8c9bd8b3").unwrap(), "  ");
        Ok(())
    }

    // cargo test -- --show-output test_string_to_map_2
    #[test]
    fn test_string_to_map_2() -> Result<()> {
        let translated_map_str = r#"
        0xf06092ac6d0 <to_translate>translate me</to_translate>      <to_translate>翻譯我</to_translate>
        0xf06092ac6d1 <to_translate>rust is great</to_translate>      <to_translate>銹很棒</to_translate>
        0xf06092ac6d2 <to_translate>let's have a weekend</to_translate>      <to_translate>讓我們週末</to_translate>
        "#;

        let translated_map = string_to_map(translated_map_str.to_string())?;
        assert_eq!(translated_map.len(), 3);

        println!("translated_map: {:#?}", translated_map);

        assert_eq!(translated_map.get("0xf06092ac6d0").unwrap(), "翻譯我");
        assert_eq!(translated_map.get("0xf06092ac6d1").unwrap(), "銹很棒");
        assert_eq!(translated_map.get("0xf06092ac6d2").unwrap(), "讓我們週末");
        Ok(())
    }

    // cargo test -- --show-output test_string_to_map_3
    #[test]
    fn test_string_to_map_3() -> Result<()> {
        let translated_map_str = r#"
        0xf06092ac6d0 <to_translate>переведите меня</to_translate>	<to_translate>ترجمة لي</to_translate>
        0xf06092ac6d1 <to_translate>ржавчина это здорово</to_translate>	<to_translate>الصدأ رائع</to_translate>
        0xf06092ac6d2 <to_translate>давай проведем выходные</to_translate>	<to_translate>لنحصل على عطلة نهاية أسبوع</to_translate>
        "#;

        let translated_map = string_to_map(translated_map_str.to_string())?;
        assert_eq!(translated_map.len(), 3);

        println!("translated_map: {:#?}", translated_map);

        assert_eq!(translated_map.get("0xf06092ac6d0").unwrap(), "ترجمة لي");
        assert_eq!(translated_map.get("0xf06092ac6d1").unwrap(), "الصدأ رائع");
        assert_eq!(
            translated_map.get("0xf06092ac6d2").unwrap(),
            "لنحصل على عطلة نهاية أسبوع"
        );
        Ok(())
    }

    // cargo test -- --show-output parse_tsv_line
    #[test]
    fn test_parse_tsv_line() {
        init_logging();
        assert_eq!(parse_tsv_line("0x2253f4530b0 <to_translate>convert it into inches</to_translate>       <to_translate>es in Zoll umwandeln</to_translate>").unwrap(), TsvLine {
            ref_addr: "0x2253f4530b0".to_owned(),
            orig_text: "convert it into inches".to_owned(),
            translated_text: "es in Zoll umwandeln".to_owned()
        });

        // google translate api sometimes puts , in front of second span :(
        assert_eq!(parse_tsv_line("0x28c2af58fd0 <to_translate>what's the currency exchange </to_translate>        , <to_translate>was der Wechsel ist</to_translate>").unwrap(), TsvLine {
            ref_addr: "0x28c2af58fd0".to_owned(),
            orig_text: "what's the currency exchange ".to_owned(),
            translated_text: "was der Wechsel ist".to_owned()
        });

        // must work also with leading white chars just for any case (e.g. to work in unit tests like test_string_to_map_1)
        assert_eq!(parse_tsv_line("     7f06092ac6d0 <to_translate>translate me</to_translate>      <to_translate>übersetze mich</to_translate>").unwrap(), TsvLine {
            ref_addr: "7f06092ac6d0".to_owned(),
            orig_text: "translate me".to_owned(),
            translated_text: "übersetze mich".to_owned()
        });
    }

    // cargo test -- --show-output test_create_sample_tsv_file
    #[test]
    #[ignore]
    fn test_create_sample_tsv_file() -> Result<()> {
        let content = "0x1bb39b97461\t<to_translate>translate me</to_translate>\n0x1bb39b97462\t<to_translate>rust is great</to_translate>\n0x1bb39b97463\t<to_translate>let's have a weekend</to_translate>\n".to_string();
        let mut agent_file_handle = File::create("c:/tmp/sample_tsv.tsv")?;
        agent_file_handle.write_all(content.as_bytes())?;
        Ok(())
    }

    // cargo test -- --show-output test_create_sample_glossary_file
    #[test]
    #[ignore]
    fn test_create_sample_glossary_file() -> Result<()> {
        /*
        $trackingId\t$trackingId\n
        $length\t$length\n
        $cityFrom\t $cityFrom\n
        $countryFrom\t$countryFrom\n
        $cityTo\t$cityTo\n
        $countryTo\t$countryTo\n
        $edd\t$edd\n
        $dispatchTerminal\t$dispatchTerminal\n
        $eventDate\t$eventDate\n
        $byeText\t$byeText\n
        $date\t$date\n
        $languageCode\t$languageCode\n
        */
        let content = "$trackingId\t$trackingId\n$length\t$length\n$cityFrom\t $cityFrom\n$countryFrom\t$countryFrom\n$cityTo\t$cityTo\n$countryTo\t$countryTo\n$edd\t$edd\n$dispatchTerminal\t$dispatchTerminal\n$eventDate\t$eventDate\n$byeText\t$byeText\n$date\t$date\n$languageCode\t$languageCode\n".to_string();
        let mut agent_file_handle = File::create("c:/tmp/sample_glossary.tsv")?;
        agent_file_handle.write_all(content.as_bytes())?;
        Ok(())
    }
}

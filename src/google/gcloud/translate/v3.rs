// https://cloud.google.com/translate/docs/intro-to-v3
// https://cloud.google.com/translate/docs/reference/rest/v3/projects/translateText
// https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations/batchTranslateText
// https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations.operations#Operation
// https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations.operations/get
// https://cloud.google.com/translate/docs/reference/rest/v3/projects.locations.operations/wait

/*

curl --location --request POST 'https://translation.googleapis.com/v3/projects/dummy-project-id/locations/us-central1:batchTranslateText' \
--header 'Authorization: Bearer ya29.c....' \
--header 'Content-Type: application/javascript' \
--data-raw '{
    "sourceLanguageCode": "en",
    "targetLanguageCodes": "de",
    "inputConfigs": [{
        "mimeType":  "text/html",
        "gcsSource": {
            "inputUri": "gs://translate_v3_test_in/input.tsv"
        }
    }],
    "outputConfig": {
        "gcsDestination": {
            "outputUriPrefix": "gs://translate_v3_test_out/"
        }
    }
}'


curl --location --request GET 'https://translation.googleapis.com/v3/projects/dummy-project-id/locations/us-central1/operations/20200615-11411592246465-5edebebf-0000-2598-9feb-24058877eccc' \
--header 'Authorization: Bearer ya29.c....'


curl --location --request POST 'https://translation.googleapis.com/v3/projects/dummy-project-id/locations/us-central1/operations/20200615-11581592247524-5edeccd9-0000-26b7-bd4f-30fd38139c64:wait' \
--header 'Authorization: Bearer ya29.c....' \
--header 'Content-Type: application/json' \
--data-raw '{
  "timeout": "60s"
}'

*/

use crate::errors::Result;
use log::debug;
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::collections;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3ResponseMetadata {
    #[serde(rename = "@type")]
    type_attr: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3Response {
    name: String,
    metadata: GoogleTranslateV3ResponseMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTranslateV3ApiResponse {
    pub status_code: String,
    pub body: GoogleTranslateV3Response,
}

pub struct GoogleTranslateV3Map {
    pub map_to_translate: collections::HashMap<String, String>,
    pub tsv_map: collections::HashMap<String, String>,
}

impl GoogleTranslateV3Map {
    pub fn new(map_to_translate: collections::HashMap<String, String>) -> Self {
        let mut tsv_map = collections::HashMap::new();
        let mut idx = 0;

        for (_, val) in map_to_translate.iter() {
            idx = idx + 1;
            tsv_map.insert(idx.to_string(), val.to_string());
        }

        GoogleTranslateV3Map {
            map_to_translate,
            tsv_map,
        }
    }

    pub fn map_to_string(translation_map: &collections::HashMap<String, String>) -> String {
        let mut s = String::from("");

        for (key, val) in translation_map.iter() {
            s.push_str(&format!("{} {}\n", key, val));
        }

        s
    }

    pub fn string_to_map(s: String) -> collections::HashMap<String, String> {
        let mut translation_map: collections::HashMap<String, String> = collections::HashMap::new();
        let split = s.split("\n");

        let vec: Vec<&str> = split.collect();

        for item in vec.iter() {
            if item.trim() == "" {
                continue; // skip the last empty row
            }
            let idx = item.find(" ").unwrap(); // safe to unwrap, we will be using with translation map only ;)
            translation_map.insert(item[0..idx].to_string(), item[idx + 1..].to_string());
        }

        translation_map
    }
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

    let response_body: GoogleTranslateV3Response = serde_json::from_str(&resp.body_string().await?)?; 

    Ok(GoogleTranslateV3ApiResponse {
        status_code: resp.status().as_str().to_string(),
        body: response_body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::google::gcloud::auth::*;
    use async_std::task;
    // cargo test -- --show-output test_batch_translate_text
    #[test]
    //#[ignore]
    fn test_batch_translate_text() -> Result<()> {

        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);


        let api_response: Result<GoogleTranslateV3ApiResponse> = task::block_on(batch_translate_text(
            &token,
            "express-tracking",
            "en", "de", "text/html",
            "gs://translate_v3_test/translation_map",
            "gs://translate_v3_test_out/",
        ));

        println!("api_response {:#?}", api_response?);
        Ok(())
    }
}

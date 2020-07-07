use crate::errors::Result;
use crate::google::gcloud::auth::*;
#[allow(unused_imports)]
use async_std::{fs, task};
use serde::{Deserialize, Serialize};
use surf;

pub enum TranslateFormat {
    Plain,
    Html,
}

#[derive(Serialize, Deserialize)]
pub struct TranslateQuery {
    q: String,
    target: String,
    format: String,
    source: String,
}

pub struct TranslateResponse {
    pub status: String,
    pub body: TranslateResponseBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateResponseBodyTranslationItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detectedSourceLanguage")]
    pub detected_source_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "translatedText")]
    pub translated_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateResponseBodyData {
    pub translations: Vec<TranslateResponseBodyTranslationItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslateResponseBody {
    pub data: TranslateResponseBodyData,
}

// see https://cloud.google.com/translate/docs/reference/rest/v2/translate
pub async fn translate(
    gdf_credentials_file: &str,
    source_lang: &str,
    target_lang: &str,
    text: &str,
    format: TranslateFormat,
) -> Result<TranslateResponse> {
    let api_url = "https://translation.googleapis.com/language/translate/v2";
    let token = get_google_api_token(gdf_credentials_file).await?;
    let token_header = format!("Bearer {}", token.access_token);

    let format_str = match format {
        TranslateFormat::Html => "html",
        TranslateFormat::Plain => "text",
    };

    let mut resp = surf::post(api_url)
        .set_header("Authorization", token_header)
        .set_query(&TranslateQuery {
            q: text.to_owned(),
            target: target_lang.to_owned(),
            format: format_str.to_owned(),
            source: source_lang.to_owned(),
        })?
        .await?;

    Ok(TranslateResponse {
        status: resp.status().as_str().to_string(),
        body: resp.body_json::<TranslateResponseBody>().await?,
    })
}

#[allow(dead_code)]
async fn sample_http_call() -> Result<String> {
    let mut res = surf::get("https://www.rust-lang.org").await?;
    Ok(res.body_string().await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test -- --show-output test_sample_http_call
    #[test]
    #[ignore]
    fn test_sample_http_call() -> Result<()> {
        let http_task = task::spawn(async {
            let result = sample_http_call().await;

            match result {
                Ok(str_val) => println!("sample_http_call ok {}", str_val),
                _ => println!("sample_http_call ko"),
            }
        });

        task::block_on(http_task);

        Ok(())
    }

    // cargo test -- --show-output test_translate
    #[test]
    #[ignore]
    fn test_translate() -> Result<()> {
        let result: Result<TranslateResponse> = task::block_on(translate(
            "./examples/testdata/credentials.json",
            "en",
            "de",
            "Rust is wonderfull programming language",
            TranslateFormat::Plain,
        ));
        println!("result from translate: {:#?}", result.unwrap().body);
        Ok(())
    }
}

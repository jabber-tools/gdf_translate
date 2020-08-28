//! # implementation of google translation api v2
//!
use crate::errors::Result;
#[allow(unused_imports)]
use async_std::{fs, task};
use log::debug;
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

#[derive(Debug)]
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
    token: &str,
    source_lang: &str,
    target_lang: &str,
    text: &str,
    format: &TranslateFormat,
) -> Result<TranslateResponse> {
    let api_url = "https://translation.googleapis.com/language/translate/v2";

    let format_str = match format {
        TranslateFormat::Html => "html",
        TranslateFormat::Plain => "text",
    };

    debug!("going to translate text {}", text);
    let mut resp = surf::post(api_url)
        .set_header("Authorization", token)
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
    use crate::google::gcloud::auth::*;

    async fn translate_v2_dummy_wrapper(
        token: &str,
        source_lang: &str,
        target_lang: &str,
        text: &str,
    ) -> Result<()> {
        let iter_vec: Vec<u8> = vec![1, 2, 3];
        for idx in iter_vec.iter() {
            println!("iteration {}", idx);
            let resp = translate(
                token,
                source_lang,
                target_lang,
                text,
                &TranslateFormat::Plain,
            )
            .await?;
            println!("resp={:#?}", resp);
        }

        Ok(())
    }

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

    // cargo test -- --show-output test_translate_v2
    #[test]
    #[ignore]
    fn test_translate_v2() -> Result<()> {
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);

        let result: Result<TranslateResponse> = task::block_on(translate(
            &token,
            "en",
            "de",
            "Rust is wonderfull programming language",
            &TranslateFormat::Plain,
        ));
        println!("result from translate: {:#?}", result.unwrap().body);
        Ok(())
    }

    // cargo test -- --show-output test_translate_v2_wrapped
    #[test]
    #[ignore]
    fn test_translate_v2_wrapped() -> Result<()> {
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);

        let _ = task::block_on(translate_v2_dummy_wrapper(
            &token,
            "en",
            "de",
            "Rust is wonderfull programming language. This is wrapped translation!",
        ));
        Ok(())
    }
}

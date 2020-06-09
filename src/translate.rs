#[allow(unused_imports)]
use crate::errors::{Error, Result};
#[allow(unused_imports)]
use async_std::{task, fs};
use serde::{Deserialize, Serialize};
use surf;

// https://cloud.google.com/translate/docs/reference/rest/v2/translate

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: u64,
    iat: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GDFCredentials {
    pub r#type: String,
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleApisOauthToken {
    pub access_token: String,
    pub token_type: String,
}

pub async fn file_to_gdf_credentials(file_name: &str) -> Result<GDFCredentials> {
    let file_str = fs::read_to_string(file_name).await?;
    let cred = serde_json::from_str::<GDFCredentials>(&file_str)?;
    Ok(cred)
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

    // cargo test -- --show-output test_file_to_gdf_credentials
    #[test]
    #[ignore]
    fn test_file_to_gdf_credentials() -> Result<()> {
        let result: Result<GDFCredentials> = task::block_on(file_to_gdf_credentials("./examples/testdata/credentials.json"));
        println!("result from file_to_gdf_credentials: {:#?}", result.unwrap());
        Ok(())
    }
}

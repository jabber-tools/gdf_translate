use crate::errors::Result;
#[allow(unused_imports)]
use async_std::{fs, task};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use surf;

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

fn new_token_from_cred(cred: &GDFCredentials) -> Result<String> {
    let _now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let claims = Claims {
        iss: cred.client_email.clone(),
        scope: "https://www.googleapis.com/auth/cloud-platform".to_owned(),
        aud: "https://www.googleapis.com/oauth2/v4/token".to_owned(),
        exp: _now + 3600,
        iat: _now,
    };

    // RS256 - encrypting with private key
    let priv_key = str_to_encoding_key(cred.private_key.clone())?;
    let token = encode(&Header::new(Algorithm::RS256), &claims, &priv_key)?;
    Ok(token)
}

fn str_to_encoding_key(priv_key_str: String) -> Result<EncodingKey> {
    let key = EncodingKey::from_rsa_pem(priv_key_str.replace("\\n", "\n").into_bytes().as_slice())?;
    Ok(key)
}

pub async fn get_google_api_token(gdf_credentials_file: &str) -> Result<GoogleApisOauthToken> {
    let cred = file_to_gdf_credentials(gdf_credentials_file).await?;
    let token = new_token_from_cred(&cred)?;

    let body = format!(
        "grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&assertion={}",
        token
    );

    let resp = surf::post("https://www.googleapis.com/oauth2/v4/token")
        .body_string(body)
        .set_header("Content-Type", "application/x-www-form-urlencoded")
        .recv_string()
        .await?;

    let google_apis_token = serde_json::from_str::<GoogleApisOauthToken>(&resp)?;
    Ok(google_apis_token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Result;

    // cargo test -- --show-output test_file_to_gdf_credentials
    #[test]
    #[ignore]
    fn test_file_to_gdf_credentials() -> Result<()> {
        let result: Result<GDFCredentials> = task::block_on(file_to_gdf_credentials(
            "./examples/testdata/credentials.json",
        ));
        println!(
            "result from file_to_gdf_credentials: {:#?}",
            result.unwrap()
        );
        Ok(())
    }

    // cargo test -- --show-output test_get_google_api_token
    #[test]
    #[ignore]
    fn test_get_google_api_token() -> Result<()> {
        let result: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        println!("result from get_google_api_token: {:#?}", result.unwrap());
        Ok(())
    }
}

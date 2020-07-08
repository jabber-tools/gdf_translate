// simple module for google cloud storage bucket management
// something like this :) https://github.com/ThouCheese/cloud-storage-rs/
// we basically just need to do followig 5 operations:
// 1. https://cloud.google.com/storage/docs/creating-buckets#storage-create-bucket-console
// 2. https://cloud.google.com/storage/docs/uploading-objects
// 3. https://cloud.google.com/storage/docs/downloading-objects
// 4. https://cloud.google.com/storage/docs/deleting-buckets
// 5. https://cloud.google.com/storage/docs/deleting-objects

use crate::errors::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status_code: String,
    pub body: String,
}

pub async fn create_bucket(
    token: &str,
    project_id: &str,
    bucket_name: &str,
    location: &str,
    storage_class: &str,
) -> Result<ApiResponse> {
    let body = json!({
        "name": bucket_name,
        "location": location,
        "storageClass": storage_class
    });

    let url = format!(
        "https://storage.googleapis.com/storage/v1/b?project={}",
        project_id
    );

    debug!("body: {}", body);
    debug!("url: {}", url);

    let mut resp = surf::post(url)
        .set_header("Authorization", token)
        .body_json(&body)?
        .await?;

    Ok(ApiResponse {
        status_code: resp.status().as_str().to_string(),
        body: resp.body_string().await?,
    })
}

/*
pub async fn upload_object(token: &str, bucket_name: &str, object_name: &str) {}

pub async fn download_object(token: &str, bucket_name: &str, object_name: &str) {}

pub async fn delete_object(token: &str, bucket_name: &str, object_name: &str) {}

*/

pub async fn delete_bucket(token: &str, bucket_name: &str) -> Result<ApiResponse> {
    let url = format!(
        "https://storage.googleapis.com/storage/v1/b/{}",
        bucket_name
    );
    debug!("url: {}", url);
    let mut resp = surf::delete(url).set_header("Authorization", token).await?;

    Ok(ApiResponse {
        status_code: resp.status().as_str().to_string(),
        body: resp.body_string().await?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::google::gcloud::auth::*;
    use async_std::task;

    #[allow(dead_code)]
    fn init_logging() {
        // enable in unit/integration tests selectivelly only when needed!
        // set RUST_LOG=gdf_translate::google::gcloud::storage_bucket_mgmt=debug
        let _ = env_logger::builder().is_test(true).try_init();
    }

    // cargo test -- --show-output test_create_bucket
    #[test]
    //#[ignore]
    fn test_create_bucket() -> Result<()> {
        init_logging();
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);

        println!("access_token {:#?}", token);
        let api_response: Result<ApiResponse> = task::block_on(create_bucket(
            &token,
            "express-tracking",
            "translate_v3_test",
            "EUROPE-WEST3",
            "STANDARD",
        ));
        println!("api_response {:#?}", api_response?);
        Ok(())
    }

    // cargo test -- --show-output test_delete_bucket
    #[test]
    //#[ignore]
    fn test_delete_bucket() -> Result<()> {
        init_logging();
        let token: Result<GoogleApisOauthToken> =
            task::block_on(get_google_api_token("./examples/testdata/credentials.json"));
        let token = format!("Bearer {}", token.unwrap().access_token);

        println!("access_token {:#?}", token);
        let api_response: Result<ApiResponse> =
            task::block_on(delete_bucket(&token, "translate_v3_test"));
        println!("api_response {:#?}", api_response?);
        Ok(())
    }
}

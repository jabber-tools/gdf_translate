use serde::{Deserialize, Serialize};

pub mod auth;
pub mod storage_bucket_mgmt;
pub mod translate;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status_code: String,
    pub body: String,
}

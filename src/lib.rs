//! Utility for translating Google DialogFlow Agents utilizing Google Translation API V2/V3
//!
//! # How does it work
//!
//! TBD...
//!
//!
pub mod cli;
pub mod errors;
pub mod google;
pub mod html;
pub mod macros;
pub mod ui;
pub mod zip;

/// Utility function to enable log::debug logging in unit tests
pub fn init_logging() {
    // enable in unit/integration tests selectivelly only when needed!
    // set RUST_LOG=gdf_translate::google::gcloud::storage_bucket_mgmt=debug
    let _ = env_logger::builder().is_test(true).try_init();
}

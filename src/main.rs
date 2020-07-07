#[allow(unused_imports)]
use gdf_translate::errors::Result;
#[allow(unused_imports)]
use gdf_translate::google::gcloud::auth::*;
#[allow(unused_imports)]
use gdf_translate::google::gcloud::storage_bucket_mgmt::*;
#[allow(unused_imports)]
use gdf_translate::google::gcloud::translate::v2::*;
#[allow(unused_imports)]
use gdf_translate::google::gcloud::translate::v3::*;

fn main() {
    env_logger::init();
    println!("Hello, GDF translate!");
}

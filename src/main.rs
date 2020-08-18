#[allow(unused_imports)]
use gdf_translate::cli::*;
use gdf_translate::cli::{get_cmd_line_parser, get_cmdl_options};
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

// cargo run -- --agent-file c:/a/b/c.zip --output-folder c:/a/b/c/d --source-lang en --target-lang de --cred-file c:/x/y/z.json --api-version v2
fn main() {
    env_logger::init();
    let cmd_line_matches = get_cmd_line_parser().get_matches();
    let cmd_line_opts = get_cmdl_options(&cmd_line_matches);
    println!("cmd_line_opts: {:#?}", cmd_line_opts);
}

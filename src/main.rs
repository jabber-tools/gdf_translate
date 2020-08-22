use async_std::task;
use gdf_translate::cli::{get_cmd_line_parser, get_cmdl_options};
use gdf_translate::errors::Result;
use gdf_translate::google::gcloud::auth::*;
use gdf_translate::google::gcloud::translate::{
    GoogleTranslateV2, GoogleTranslateV3, TranslationProviders,
};
use gdf_translate::ui::{ProgressMessageType, UserInterface};
use std::process;
use std::sync::mpsc::channel;
use std::time::Instant;

// cargo run -- --agent-file C:/Users/adamb/adam/_DEV/rust/projects/gdf_translate/examples/sample_agents/Currency-Converter.zip --output-folder c:/tmp/out --source-lang en --target-lang de --cred-file C:/Users/adamb/adam/_DEV/rust/projects/gdf_translate/examples/testdata/credentials.json
// cargo run -- --agent-file C:/Users/adamb/adam/_DEV/rust/projects/gdf_translate/examples/sample_agents/Currency-Converter.zip --output-folder c:/tmp/out --source-lang en --target-lang de --cred-file C:/Users/adamb/adam/_DEV/rust/projects/gdf_translate/examples/testdata/credentials.json --api-version v2
// cargo run -- --agent-file C:/Users/adamb/adam/_DEV/rust/projects/gdf_translate/examples/sample_agents/Currency-Converter.zip --output-folder c:/tmp/out --source-lang en --target-lang de --cred-file C:/Users/adamb/adam/_DEV/rust/projects/gdf_translate/examples/testdata/credentials.json --api-version v3 --create-output-tsv
fn main() {
    env_logger::init();
    let cmd_line_matches = get_cmd_line_parser().get_matches();
    let cmd_line_opts = get_cmdl_options(&cmd_line_matches);
    // println!("cmd_line_opts: {:#?}", cmd_line_opts);

    let token: Result<GoogleApisOauthToken> = task::block_on(get_google_api_token(
        cmd_line_opts.gcloud_svc_acc_cred.to_str().unwrap(), // TBD: do not unwrap and provide proper err msg in case if None value!
    ));
    let token = format!("Bearer {}", token.unwrap().access_token);

    let gdf_credentials = task::block_on(file_to_gdf_credentials(
        cmd_line_opts.gcloud_svc_acc_cred.to_str().unwrap(),
    ));

    if let Err(some_error) = gdf_credentials {
        println!(
            "unable to parse credenials file due to following error: {:#?}",
            some_error
        );
        process::exit(1);
    }
    let gdf_credentials = gdf_credentials.unwrap();
    /* println!(
        "gdf_credentials.project_id: {:#?}",
        gdf_credentials.project_id
    ); */

    let (tx, rx) = channel::<ProgressMessageType>();
    let mut ui;
    match cmd_line_opts.translation_mode {
        TranslationProviders::GoogleTranslateV2 => {
            ui = UserInterface::new(rx, TranslationProviders::GoogleTranslateV2)
        }
        TranslationProviders::GoogleTranslateV3 => {
            ui = UserInterface::new(rx, TranslationProviders::GoogleTranslateV3)
        }
        _ => unreachable!(),
    }

    std::thread::spawn(move || {
        ui.progress_update_handler();
    });

    match cmd_line_opts.translation_mode {
        TranslationProviders::GoogleTranslateV2 => {
            println!("Starting V2 translation...");
            let start = Instant::now();
            let _ = task::block_on(GoogleTranslateV2::execute_translation(
                cmd_line_opts.gdf_agent_zip_path.to_str().unwrap(),
                cmd_line_opts.output_folder.to_str().unwrap(),
                &token,
                &cmd_line_opts.from_lang,
                &cmd_line_opts.to_lang,
                tx,
            ));
            let duration = start.elapsed();
            println!("Translation done! Total duration: {:?}", duration);
        }
        TranslationProviders::GoogleTranslateV3 => {
            println!("Starting V3 translation...");
            let start = Instant::now();
            let _ = task::block_on(GoogleTranslateV3::execute_translation(
                cmd_line_opts.gdf_agent_zip_path.to_str().unwrap(),
                cmd_line_opts.output_folder.to_str().unwrap(),
                &token,
                &cmd_line_opts.from_lang,
                &cmd_line_opts.to_lang,
                &gdf_credentials.project_id,
                tx,
                cmd_line_opts.create_output_tsv,
            ));
            let duration = start.elapsed();
            println!("Translation done! Total duration: {:?}", duration);
        }
        _ => unreachable!(),
    }
}

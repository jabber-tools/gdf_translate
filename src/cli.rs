use crate::google::gcloud::translate::TranslationProviders;
use clap::{App, Arg, ArgMatches};
use std::path::Path;

// TBD: include flag for creation of file bucket_download_result.txt (for debugging only)
#[derive(Debug)]
pub struct CommandLine<'a> {
    pub gdf_agent_zip_path: &'a Path,
    pub output_folder: &'a Path,
    pub from_lang: String,
    pub to_lang: String,
    pub gcloud_svc_acc_cred: &'a Path,
    pub translation_mode: TranslationProviders,
}

impl<'a> CommandLine<'a> {
    fn new(
        gdf_agent_zip_path: &'a Path,
        output_folder: &'a Path,
        from_lang: String,
        to_lang: String,
        gcloud_svc_acc_cred: &'a Path,
        translation_mode: TranslationProviders,
    ) -> Self {
        CommandLine {
            gdf_agent_zip_path,
            output_folder,
            from_lang,
            to_lang,
            gcloud_svc_acc_cred,
            translation_mode,
        }
    }
}

pub fn get_cmd_line_parser<'a, 'b>() -> App<'a, 'b> {
    App::new("Google DialogFlow Translate")
        .version("0.1.0")
        .author("Adam Bezecny")
        .about("Tool for automated translation of Google DialogFlow agents.")
        .arg(
            Arg::with_name("gdf_agent_zip_path")
                .short("f")
                .long("agent-file")
                .value_name("FILE")
                .help("ZIP file with exported GDF agent")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("output_folder")
                .short("o")
                .long("output-folder")
                .value_name("FOLDER")
                .help("Path to folder where translated agent will be stored. Must be exiting (ideally empty) folder.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("from_lang")
                .short("s")
                .long("source-lang")
                .value_name("lang ISO code")
                .help("ISO code of source language.E.g.: en")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("to_lang")
                .short("t")
                .long("target-lang")
                .value_name("lang ISO code")
                .help("ISO code of destination/target language to which agent will be translated .E.g.: de")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("gcloud_svc_acc_cred")
                .short("c")
                .long("cred-file")
                .value_name("FILE")
                .help("Path to Google Cloud service account credentials used to run translation via Google Translate V2/V3 API. Must have respective priviledges: TBD...")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("translation_mode")
                .short("a")
                .long("api-version")
                .value_name("V2/V3")
                .help("Version of API used to translate. Can be v2/v3. If not specified defaults to v3.")
                .takes_value(true)
                .possible_values(&["v2", "v3"])
                .default_value("v3")
        )
}

pub fn get_cmdl_options<'a>(matches: &'a ArgMatches) -> CommandLine<'a> {
    let translation_mode;

    // safe to unwrap 5 belove listed params since they are defined for clap as required required!
    let gdf_agent_zip_path = Path::new(matches.value_of("gdf_agent_zip_path").unwrap());
    let output_folder = Path::new(matches.value_of("output_folder").unwrap());
    let from_lang = matches.value_of("from_lang").unwrap();
    let to_lang = matches.value_of("to_lang").unwrap();
    let gcloud_svc_acc_cred = Path::new(matches.value_of("gcloud_svc_acc_cred").unwrap());

    if let Some(val) = matches.value_of("translation_mode") {
        match val {
            "v2" | "V2" => translation_mode = TranslationProviders::GoogleTranslateV2,
            "v3" | "V3" => translation_mode = TranslationProviders::GoogleTranslateV3,
            _ => unreachable!(),
        }
    } else {
        translation_mode = TranslationProviders::GoogleTranslateV3;
    }

    CommandLine::new(
        gdf_agent_zip_path,
        output_folder,
        from_lang.to_owned(),
        to_lang.to_owned(),
        gcloud_svc_acc_cred,
        translation_mode,
    )
}

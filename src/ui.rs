use crate::google::gcloud::translate::TranslationProviders;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::mpsc;

pub enum ProgressMessageType {
    V2CountSpecified(u64),
    ItemProcessed,
    TextMessage(String),
    Exit,
}

pub fn get_progress_bar(count: u64) -> ProgressBar {
    let pb_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:70.yellow/red} {pos:>7}/{len:7} {msg}")
        .progress_chars("##-");

    let pb = ProgressBar::new(count);
    pb.set_style(pb_style);
    pb.set_position(1);
    pb
}

fn progress_update_handler_v2(rx: mpsc::Receiver<ProgressMessageType>, pb: ProgressBar) {
    loop {
        let msg = rx.recv().unwrap();
        match msg {
            ProgressMessageType::TextMessage(msg) => println!("{}", msg),
            ProgressMessageType::V2CountSpecified(translation_count) => {
                pb.set_length(translation_count)
            }
            ProgressMessageType::ItemProcessed => pb.inc(1),
            ProgressMessageType::Exit => break,
            _ => unreachable!(),
        }
    }
}

fn progress_update_handler_v3(rx: mpsc::Receiver<ProgressMessageType>, pb: ProgressBar) {
    loop {
        let msg = rx.recv().unwrap();
        match msg {
            ProgressMessageType::TextMessage(msg) => println!("{}", msg),
            ProgressMessageType::Exit => break,
            _ => unreachable!(),
        }
    }
}

pub fn progress_update_handler(
    rx: mpsc::Receiver<ProgressMessageType>,
    pb: ProgressBar,
    tran_provider: TranslationProviders,
) {
    match tran_provider {
        TranslationProviders::GoogleTranslateV2 => progress_update_handler_v2(rx, pb),
        TranslationProviders::GoogleTranslateV3 => progress_update_handler_v3(rx, pb),
        _ => unreachable!(),
    }
}

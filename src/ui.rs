use crate::google::gcloud::translate::TranslationProviders;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::mpsc;

pub enum ProgressMessageType {
    V2CountSpecified(u64),
    ItemProcessed,
    TextMessage(String),
    Exit,
}

pub struct UserInterface {
    progress_bar: Option<ProgressBar>,
    mpsc_receiver: mpsc::Receiver<ProgressMessageType>,
    api_version: TranslationProviders,
}

impl UserInterface {
    pub fn new(rx: mpsc::Receiver<ProgressMessageType>, api_version: TranslationProviders) -> Self {
        UserInterface {
            progress_bar: None,
            mpsc_receiver: rx,
            api_version: api_version,
        }
    }

    pub fn add_progress_bar(&mut self, count: u64) {
        let pb_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:70.yellow/red} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-");

        let pb = ProgressBar::new(count);
        pb.set_style(pb_style);
        pb.set_position(0);
        self.progress_bar = Some(pb);
    }

    fn progress_update_handler_v2(&mut self) {
        loop {
            let msg = self.mpsc_receiver.recv().unwrap();
            match msg {
                ProgressMessageType::TextMessage(msg) => println!("{}", msg),
                ProgressMessageType::V2CountSpecified(translation_count) => {
                    if let None = self.progress_bar {
                        self.add_progress_bar(translation_count);
                    }
                }
                ProgressMessageType::ItemProcessed => {
                    if let Some(pb) = &self.progress_bar {
                        pb.inc(1);
                    }
                }
                ProgressMessageType::Exit => break,
                _ => unreachable!(),
            }
        }
    }

    fn progress_update_handler_v3(&self) {
        loop {
            let msg = self.mpsc_receiver.recv().unwrap();
            match msg {
                ProgressMessageType::TextMessage(msg) => println!("{}", msg),
                ProgressMessageType::Exit => break,
                _ => unreachable!(),
            }
        }
    }

    pub fn progress_update_handler(&mut self) {
        match self.api_version {
            TranslationProviders::GoogleTranslateV2 => self.progress_update_handler_v2(),
            TranslationProviders::GoogleTranslateV3 => self.progress_update_handler_v3(),
            _ => unreachable!(),
        }
    }
}

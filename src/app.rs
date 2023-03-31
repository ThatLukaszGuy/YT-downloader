#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
use std::{process};
use crate::helpers::{
    formats::Formats,
    fetch::fetch_url,
    errortext::show
};
use eframe::egui;
use super::url::YoutubeUrl;
use std::sync::mpsc::{Receiver, Sender};

use egui::Color32;

pub struct DownloaderApp {
    tx: Sender<YoutubeUrl>,
    rx: Receiver<YoutubeUrl>,
    tx_download:Sender<String>,
    rx_download:Receiver<String>,
    is_fetching:bool,
    is_downloading: bool,
    display_dropdown: bool,
    finished: bool,
    link: String,
    list: Vec<(Formats, String)>,
    chosen_format: (Formats, String),
    title: String,
    path: String,
}


impl DownloaderApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for DownloaderApp {
    fn default() -> Self {
        // thread comm. channel for fetching thread
        let (tx, rx) = std::sync::mpsc::channel();
        // thread comm. channel for downloading thread
        let (tx_download, rx_download) = std::sync::mpsc::channel();
        
        Self {
            tx,
            rx,
            tx_download,
            rx_download,
            is_fetching: false,
            is_downloading: false,
            display_dropdown: false,
            finished: false,
            link: "".to_owned(),
            list: vec![(Formats::NO_URL,"No link".to_owned())],
            chosen_format: (Formats::NO_URL,"No link".to_owned()),
            title: String::new(),
            path: "No path".to_owned()
        }
    }
}



impl eframe::App for DownloaderApp {
   

    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // if valid link fetches html
        if let Ok(yt) = self.rx.try_recv() {
            self.list = yt.do_all();
            self.is_fetching = false;
            self.display_dropdown = true
        }

        // if file gets downloaded
        if let Ok(str) = self.rx_download.try_recv() {
            self.is_downloading = false;
            self.path = str;
            self.finished = true
        }



        // footer with loader
        egui::TopBottomPanel::bottom("finished").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(15.0);
                if self.is_fetching == true {
                    ui.label("Fetching html...");
                    ui.add_space(10.0);
                    ui.spinner();
                } else if self.is_downloading == true {
                    ui.label("Downloading file, this may take from seconds to minutes depending on file size, quality, format etc.");
                    ui.add_space(10.0);
                    ui.spinner();
                } else if self.finished == true {
                    ui.label(format!("File Downloaded at: {}", self.path));
                } else {
                    ui.label("waiting...");
                }
                ui.add_space(15.0);
            });
        
        });

        // header
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(15.0);
                ui.heading("YT-Downloader");
                ui.label("Submit a link, wait, choose an available format and download.");
                ui.add_space(15.0);
            });

         });

        // input 
        egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.vertical_centered_justified(|ui| {
                // link input
                ui.add(egui::TextEdit::singleline(&mut self.link).hint_text("Input Youtube url:"));
                // title input 
                ui.add(egui::TextEdit::singleline(&mut self.title).hint_text("Save file under this name (min. 5 characters):"));
                // fetch for input link
                if ui.button("Submit").clicked() && self.is_fetching == false && self.link != "".to_owned() && self.title.chars().count() >= 5  {
                    self.is_fetching = true;
                    call(self.tx.clone(), ctx.clone(), self.link.clone());
                    
                    // revert input vals
                    self.link = "".to_owned(); 
                }
            });

            ui.horizontal(|ui| {

                // display format dropdown box
                if self.display_dropdown == true && self.is_downloading == false {
                    egui::ComboBox::from_label("Choose Format")
                    .selected_text(self.chosen_format.0.to_string()).show_ui(ui, |ui| {
                            for t in &self.list {
                                ui.selectable_value(&mut self.chosen_format, t.clone(), &t.0.to_string() );
                            }
                        }
                    );
                }

                // begin download of file
                if self.display_dropdown == true  && ui.button("Download").clicked() && self.is_fetching == false  {
                    self.is_downloading = true;
                    download(self.tx_download.clone(), ctx.clone(), self.chosen_format.clone(), self.title.clone());
                    self.title = "".to_owned();
                }       
            });

            let faded_color = ui.visuals().window_fill();
            let faded_color = |color: Color32| -> Color32 {
                use egui::Rgba;
                let t = 0.8;
                egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
            };

            ui.painter().rect_filled(
                ui.available_rect_before_wrap(),
                10.0,
                faded_color(Color32::DARK_GRAY),
            );
            ui.colored_label(Color32::from_rgb(128, 140, 255), 
            " 
    Little Help:\n
    _VIDEO:  Video will be downloaded without audio
    HDR:  Hdr format
    Appended number (e.g. _2, _3, etc):  Same video format
    corresponds to multiple itags available for chosen video
            "
            ); 

        });

    }



}
// async fetch html
fn call(tx: Sender<YoutubeUrl>, ctx: egui::Context, link: String) {
    tokio::spawn(async move {
        let yt = YoutubeUrl::new(&link).await.unwrap_or_else(|_x|{
            show("Could not use this invalid link.. Sorry");
            process::exit(1);
        });

        let _ = tx.send(yt);
        ctx.request_repaint();                                        
    });
}

// async download file
fn download(tx_download: Sender<String>,ctx: egui::Context, format: (Formats, String), title:String) {
    tokio::spawn(async move {
        
        
        let str = match format.0.to_string() {
            s if s.contains("3GP") => fetch_url(&format.1, title, ".3gp").await.expect("\nCould not download URL from created link"),
            s if s.contains("FLV") => fetch_url(&format.1, title, ".flv").await.expect("\nCould not download URL from created link"),
            s if s.contains("MP4") => fetch_url(&format.1, title, ".mp4").await.expect("\nCould not download URL from created link"),
            s if s.contains("HLS") => fetch_url(&format.1, title, ".hls").await.expect("\nCould not download URL from created link"),
            s if s.contains("WEBM") => fetch_url(&format.1, title, ".webm").await.expect("\nCould not download URL from created link"),
            s if s.contains("M4A") => fetch_url(&format.1, title, ".m4a").await.expect("\nCould not download URL from created link"),
            // for now this will never run as there are no format enums with mp3
            s if s.contains("MP3") => fetch_url(&format.1, title, ".mp3").await.expect("\nCould not download URL from created link"),
            _ => "".to_owned()
        };
        
        let _ = tx_download.send(str);
        ctx.request_repaint();    
    });
}
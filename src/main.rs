#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
use uuid::Uuid;
pub mod helpers;
mod url;
use std::{process};
use crate::helpers::{
    formats::Formats,
    dir_checks::dir_check,
    fetch::fetch_url,
    errortext::show
};
use image;
use eframe::egui;
use url::YoutubeUrl;
use std::sync::mpsc::{Receiver, Sender};


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[tokio::main]
async fn main() -> Result<()> {

    // run test to check whether needed directories exist
    dir_check("\\needed_to_get_url");
    dir_check("\\ready_video_output");
    dir_check("\\downloader_img_assets");

    // gui thread init
    let native_options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(egui::Vec2 { x: 400.0, y: 500.0 }),
        icon_data: Some(load_icon(".\\downloader_img_assets\\logo.png")), 
        ..Default::default()
    };

    eframe::run_native("YT downloader", native_options, Box::new(|cc| Box::new(DownloaderApp::new(cc))));

    // needed for tokio
    Ok(())
}


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
    title: Uuid,
    path: String,
}


impl DownloaderApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        
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
            title: Uuid::new_v4(),
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
                if self.is_fetching == true {
                    ui.label("Fetching html...");
                    ui.spinner()
                } else if self.is_downloading == true {
                    ui.label("Downloading file, this may take from seconds to minutes depending on file size, quality, format etc.");
                    ui.spinner()
                } else if self.finished == true {
                    ui.label(format!("File Downloaded at: {}", self.path))
                } else {
                    ui.label("waiting...")
                }
            });
        
        });
        // header
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("YT-Downloader");
                ui.label("Submit a link, wait, choose an available format and download.");
            });
         });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.link).hint_text("Input Youtube url:"));
                // fetch for input link
                if ui.button("Submit").clicked() && self.is_fetching == false && self.link != "".to_owned() {
                    
                    println!("Url: {}", self.link);
    
                    self.is_fetching = true;
                    call(self.tx.clone(), ctx.clone(), self.link.clone());
                    self.link = "".to_owned(); 
                }
            });

            // display format dropdown box
            if self.display_dropdown == true {
                egui::ComboBox::from_label("Choose Format")
                .selected_text(self.chosen_format.0.to_string()).show_ui(ui, |ui| {
                        for t in &self.list {
                            ui.selectable_value(&mut self.chosen_format, t.clone(), &t.0.to_string() );
                        }
                    }
                );
            }
            // begin download of file
            if self.display_dropdown == true  && ui.button("Download").clicked() && self.is_fetching == false {
                self.is_downloading = true;
                download(self.tx_download.clone(), ctx.clone(), self.chosen_format.clone(), self.title.to_string());
            }             

          
        });

    }



}

fn call(tx: Sender<YoutubeUrl>, ctx: egui::Context, link: String) {
    tokio::spawn(async move {
        let yt = url::YoutubeUrl::new(&link).await.unwrap_or_else(|_x|{
            show("Could not use this invalid link.. Sorry");
            process::exit(1);
        });

        let _ = tx.send(yt);
        ctx.request_repaint();                                        
    });
}

fn download(tx_download: Sender<String>,ctx: egui::Context, format: (Formats, String), title:String) {
    tokio::spawn(async move {
        
        let str = match format.0.to_string() {
            s if s.contains("3GP") => fetch_url(&format.1, title, ".3gp").await.expect("\nCould not download URL from created link"),
            s if s.contains("FLV") => fetch_url(&format.1, title, ".flv").await.expect("\nCould not download URL from created link"),
            s if s.contains("MP4") => fetch_url(&format.1, title, ".mp4").await.expect("\nCould not download URL from created link"),
            s if s.contains("HLS") => fetch_url(&format.1, title, ".hls").await.expect("\nCould not download URL from created link"),
            s if s.contains("WEBM") => fetch_url(&format.1, title, ".webm").await.expect("\nCould not download URL from created link"),
            s if s.contains("M4A") => fetch_url(&format.1, title, ".m4a").await.expect("\nCould not download URL from created link"),
            
            _ => "".to_owned()
        };
        
        let _ = tx_download.send(str);
        ctx.request_repaint();    
    });
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
pub mod helpers;
mod url;
mod app;
use app::DownloaderApp;
use crate::helpers::{
    dir_checks::dir_check,
    loadimg::load_icon
};
use eframe::egui;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[tokio::main]
async fn main() -> Result<()> {

    // run test to check whether needed directories exist
    dir_check("\\needed_to_get_url");
    dir_check("\\ready_video_output");
    dir_check("\\downloader_img_assets");

    // gui config
    let native_options = eframe::NativeOptions {
        resizable: false,
        initial_window_size: Some(egui::Vec2 { x: 350.0, y: 400.0 }),
        //icon_data: Some(load_icon(".\\downloader_img_assets\\logo.jpg")), 
        ..Default::default()
    };

    // gui thread init
    eframe::run_native("YT downloader", native_options, Box::new(|cc| Box::new(DownloaderApp::new(cc))));

    // needed for tokio
    Ok(())
}

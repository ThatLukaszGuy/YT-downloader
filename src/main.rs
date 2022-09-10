pub mod helpers;
mod url;
use std::process;
use crate::helpers::{
    user_input::get_input,
    formats::Formats,
    dir_checks::dir_check,
    fetch::fetch_url,
    errortext::show,
    banner::banner
};
use colored::*;
use loading::Loading;
use requestty::{Question,Answer, ListItem};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[tokio::main]
async fn main() -> Result<()> {

    // run test to check whether needed directories exist
    dir_check("\\needed_to_get_url");
    dir_check("\\ready_video_output");

    // greeting
    banner();
    // first input
    let input: String = get_input("\nPlease input Youtube-Url:");
    println!("\n{} {}","Your url:".white(),input.cyan().bold());
    let title = get_input("\nWhat name should the saved file have?");
    println!("\n{} {}","Your title:".white(),title.cyan().bold());

    let yt = url::YoutubeUrl::new(&input).await.unwrap_or_else(|_x|{
        show("Could not use this invalid link.. Sorry");
        process::exit(1);
    });
    let list = yt.do_all();
    
    let mut choices: Vec<String> = vec![]; 
    for (_usize, item) in list.iter().enumerate() {
        choices.push(item.0.to_string().into());
    };

    let question = Question::select("theme")
        .message("Choose an available video format:")
        .choices(choices)
        .build();
     
    let answer = requestty::prompt_one(question).unwrap_or_else(|_x|{
        show("Could not generate format select.. Sorry");
        process::exit(1);
    });

    let loading = Loading::default();
    loading.text("Downloading file, this may take a few minutes depending on video length, format and quality. - Don't cancel this process!");
    
    if let Answer::ListItem(ListItem { index: _, text}) = answer {
        let download_link_and_format: Vec<(Formats, String)> = list
            .into_iter()
            .filter(|x| x.0.to_string() == text.to_string())
            .collect();
        
        // if no link
        if download_link_and_format[0].1 == "No Link" {
            show("None of the available formats for this video are currently supported.. Sorry");
            process::exit(1);
        }
        // fetch and download
        if download_link_and_format[0].0.to_string().contains("MP4") {
            fetch_url(download_link_and_format[0].1.clone(), title, ".mp4").await.expect("\nCould not download URL from created link");
        } else if download_link_and_format[0].0.to_string().contains("WEBM") {
            fetch_url(download_link_and_format[0].1.clone(), title, ".webm").await.expect("\nCould not download URL from created link");
        }

        loading.end()
    }
    // needed for tokio
    Ok(())
}


 
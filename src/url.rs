use reqwest;
use uuid::Uuid;
use std::process;
use crate::helpers::{
    path_creation::make_path,
    parse_strings::parse_str,
    formats::Formats,
    clean_up::clean_up_files,
    errortext::show
};

#[derive( Clone)]
pub struct YoutubeUrl {
    download_url_list: Vec<String>,
    path: String,
    pub available_formats: Vec<(Formats, String)>
}
impl YoutubeUrl {
    
    pub async fn new(input: &str) -> Result<YoutubeUrl, Box<dyn std::error::Error>>  {
        let filename = Uuid::new_v4();
        let full_path = make_path(&filename.to_string());
    
        let resp = reqwest::get(input)
            .await?
            .text()
            .await?;
        
        // write result of req to temp file
        std::fs::write(&full_path, &resp).unwrap_or_else(|_x|{
            show("Could not write to temporary file.. Maybe try running as administrator?");
            process::exit(1);
        });
        
        Ok(YoutubeUrl {
            download_url_list: Vec::new(),
            path: full_path,
            available_formats: Vec::new()
        })
    }

    pub fn do_all(mut self) -> Vec<(Formats, std::string::String)> {
        self.extract_valid_link_list();
        clean_up_files(&self.path).unwrap_or_else(|_x|{
            show("Could not clean up files.. some internal error");
            process::exit(1);
        });
        let list = self.get_formats();
        list
    }

    pub fn extract_valid_link_list(&mut self) {
        // read contents of file to var
        let first_extract = std::fs::read_to_string(&self.path).unwrap_or_else(|_x|{
            show("Could not read temporary file.. Maybe try running as administrator?");
            process::exit(1);
        });
        // place \n before all http lines
        let organize = str::replace(&first_extract, "http", "\nhttp");

        let first_list = parse_str(&organize, String::new(), "googlevideo");

        // all double quotes to newlines
        let second_organize = str::replace(&first_list, '"', "\n");
        // grep ... again
        let second_list = parse_str(&second_organize, String::new(), "googlevideo");
        let third_list = parse_str(&second_list, String::new(), "videoplayback");
        // convert to actual & char
        let added_ampersand:String = str::replace(&third_list, "\\u0026", "&");
        // putting it together
        let fourth_list = added_ampersand.replace("\'", "");

        std::fs::write(&self.path, &fourth_list.replace("https", "\nhttps")).unwrap_or_else(|_x|{
            show("Could not write to temporary file.. Maybe try running as administrator?");
            process::exit(1);
        });
        let extract = std::fs::read_to_string(&self.path).unwrap_or_else(|_x|{
            show("Could not read temporary file.. Maybe try running as administrator?");
            process::exit(1);
        });
        let final_link_list = parse_str(&extract, String::new(), "itag=");
        // last link list 
        std::fs::write(&self.path, &final_link_list.replace("https", "\nhttps")).unwrap_or_else(|_x|{
            show("Could not write to temporary file.. Maybe try running as administrator?");
            process::exit(1);
        });

        
        for line in std::fs::read_to_string(&self.path)
        .unwrap_or_else(|_x|{
            show("Could not write to temporary file.. Maybe try running as administrator?");
            process::exit(1);
        })
        .lines() {
            self.download_url_list.push(line.to_string())
        }
        
        

    }

    pub fn get_formats(mut self) -> Vec<(Formats, std::string::String)> {
        
        let mut available_formats: Vec<(Formats,String)> = Vec::new();

        
        for link in self.download_url_list {
            // only for mp4 files for now
            if link.contains("itag=18") {
                available_formats.push((Formats::MP4_360, link.clone()))
            }
            if link.contains("itag=22") {
                available_formats.push((Formats::MP4_720,link.clone()))
            }

            if link.contains("itag=37") {
                available_formats.push((Formats::MP4_1080,link.clone()))
            }

            if link.contains("itag=38") {
                available_formats.push((Formats::MP4_3072,link.clone()))
            }

            if link.contains("itag=82") {
                available_formats.push((Formats::MP4_360_3D,link.clone()))
            }

            if link.contains("itag=83") {
                available_formats.push((Formats::MP4_480_3D,link.clone()))
            }

            if link.contains("itag=84") {
                available_formats.push((Formats::MP4_720_3D,link.clone()))
            }

            if link.contains("itag=85") {
                available_formats.push((Formats::MP4_1080_3D,link.clone()))
            }
            
        }


        if available_formats.is_empty() {
            println!("This Youtube video is only available in formats that are not supported yet.");
            self.available_formats = vec![(Formats::NO_URL, "No Link".to_owned())];
            self.available_formats
        } else {
            self.available_formats = available_formats;
            self.available_formats
        }
    }
}


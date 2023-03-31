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
use strum::IntoEnumIterator;


#[derive(Clone)]
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
            show("Could not read from temporary file.. Maybe try running as administrator?");
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
            show("Could not read from temporary file.. Maybe try running as administrator?");
            process::exit(1);
        })
        .lines() {
            self.download_url_list.push(line.to_string())
        }
        
        

    }

    pub fn get_formats(mut self) -> Vec<(Formats, std::string::String)> {
        
        let mut available_formats: Vec<(Formats,String)> = Vec::new();
        
        // IMPORTANT -> same order as corresponding format
        let itag_codes: Vec<&str> = vec![
        "No link", 
        "17", "36" , 
        "5", "6","34","35", 
        "160","18","22","37","38","82","83","84","85", "133","134", "135" , "136","298", "137","299","399", "264", "400","138", "266","401","402",
        "151","92","93","94","95","96", "132",
        "43", "44","45","46", "100", "101", "102", "219", "278","330","242", "167", "243", "168", "218", "244", "245", "246", "247", "302","248", "303","271", "308", "313", "315" ,"272",
        "171","249","250","251",
        "331","332", "333", "334", "335", "336", "337",
        "139","140","141"
        
        ];
        
        let mut count = 0;
        
        for link in self.download_url_list {
            
            for format in Formats::iter() {
                
                let tag = format!("itag={}",itag_codes[count]);
                if link.contains(&tag) {
                    available_formats.push((format, link.clone()));
                }

                count = count + 1;
                // so index doesn't go out of bounds
                if count == itag_codes.len() {
                    count = 0;
                }
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


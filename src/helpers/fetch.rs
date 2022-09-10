use loading::Loading;
use tokio::{
    io::AsyncWriteExt,
    fs::File
};
use colored::*;
use tokio_stream::StreamExt;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_url(url: String, file_name: String, format: &str) -> Result<()> {



    let x = format!("{}{}{}", "ready_video_output/", file_name, format); // for now since its just mp4 files
    
    let mut file = File::create(&x).await?;
    let mut stream = reqwest::get(url)
    .await?
    .bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
    }

    file.flush().await?;
    
    

    let vid_location = format!("{}{}{}{}",
        project_root::get_project_root().unwrap().to_str().unwrap(),
        "\\ready_video_output\\",
        file_name,
        format);
    let loading = Loading::default();

    loading.success(format!("{} {}","Downloaded Successfully at".green().bold(), vid_location.green().bold() ));
    
    Ok(loading.end())
}



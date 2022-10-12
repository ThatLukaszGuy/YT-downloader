use tokio::{
    io::AsyncWriteExt,
    fs::File
};
use tokio_stream::StreamExt;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_url(url: &str, file_name: String, format: &str) -> Result<String> {



    let x = format!("{}{}{}", "ready_video_output/", file_name, format); // for now since its just mp4 files
    
    let mut file = File::create(&x).await?;
    let mut stream = reqwest::get(url.to_owned())
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
    
    Ok(vid_location)
}



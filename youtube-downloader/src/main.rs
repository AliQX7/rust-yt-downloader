use rustube::{Id, VideoFetcher};
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.youtube.com/watch?v=aqz-KE-bpKQ";

    let id = Id::from_raw(url)?;
    let descrambler = VideoFetcher::from_id(id.into_owned())?.fetch().await?;
    let video = descrambler.descramble()?;

    let stream = video
        .streams()
        .iter()
        .filter(|s| s.includes_video_track && s.includes_audio_track)
        .max_by_key(|s| s.quality_label.clone())
        .ok_or("No suitable stream found")?;

    println!("Downloading: {}", video.video_details().title);
    stream.download().await?;

    println!("Download complete!");
    Ok(())
}
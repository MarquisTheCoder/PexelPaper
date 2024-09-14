
use std::path::Path;
use ffmpeg_frame_grabber::{FFMpegVideo, FFMpegVideoOptions};

pub fn grab_wallpaper_image(wallpaper_image_path: &str) -> &'static str{

    let video = FFMpegVideo::open(
        Path::new(&"./data/video.mp4"),
        FFMpegVideoOptions::default().with_sampling_interval(Duration::from_secs(0x01)),
    )
    .unwrap();

    let frame = video[0].unwrap();
}

fn main(){

}
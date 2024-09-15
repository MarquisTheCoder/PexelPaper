

extern crate base64;

use std::process::Command;
use base64::encode;

/*ffmpeg -loglevel quiet -ss 26 -i 3196505-sd_960_540_30fps.mp4 -t 1  -f image2 - */

#[tauri::command]
pub fn raw_data_to_base64(wallpaper_video_path: &str) -> String{
    let raw_image_output = Command::new("ffmpeg")
        .args(["-loglevel", "quiet", "-ss", "26", "-i", wallpaper_video_path, "-t", "1", "-f", "image2", "-"])
        .output()
        .expect("failed to grab image");

     // Encode the binary image data as base64
     let base64_image = encode(&raw_image_output.stdout);
     return base64_image;
    
}


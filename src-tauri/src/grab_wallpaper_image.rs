
extern crate base64;
use base64::{encode, decode};

use std::process::Command;
use std::string::FromUtf8Error;

use std::io;

/*ffmpeg -loglevel quiet -ss 26 -i 3196505-sd_960_540_30fps.mp4 -t 1  -f image2 - */
pub fn grab_wallpaper_image(wallpaper_video_path: &str) -> Result<String, FromUtf8Error>{
    let raw_image_output = Command::new("ffmpeg")
        .args(["-loglevel", "quiet", "-ss", "26", "-i", wallpaper_video_path, "-t", "1", "-f", "image2", "-"])
        .output()
        .expect("failed to grab image");

     // Encode the binary image data as base64
     let base64_image = base64::encode(raw_image_output.stdout);
    
     // Return the base64-encoded string
     Ok(base64_image)
    
}

fn main(){
    let wallpaper_result = grab_wallpaper_image("/Users/coder/Wallpapers/3196505-sd_960_540_30fps.mp4");

    match wallpaper_result {
        Ok(wallpaper) => println!("Wallpaper data: {}", wallpaper),
        Err(e) => println!("Error: {}", e),
    }
    
}


extern crate base64;

use std::process::Command;
use std::string::FromUtf8Error;
use base64::encode;
use std::io;

/*ffmpeg -loglevel quiet -ss 26 -i 3196505-sd_960_540_30fps.mp4 -t 1  -f image2 - */
pub fn grab_wallpaper_image(wallpaper_video_path: &str){
    let raw_image_output = Command::new("ffmpeg")
        .args(["-loglevel", "quiet", "-ss", "26", "-i", wallpaper_video_path, "-t", "1", "-f", "image2", "-"])
        .output()
        .expect("failed to grab image");

     // Encode the binary image data as base64
     let base64_image = encode(&raw_image_output.stdout);
     println!("{}", base64_image);
    
}

fn main(){
    let _wallpaper_result = grab_wallpaper_image("/Users/coder/Wallpapers/3196505-sd_960_540_30fps.mp4");

    
}
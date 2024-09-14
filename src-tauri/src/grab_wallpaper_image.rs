
use std::process::Command;
use std::string::FromUtf8Error;


/*ffmpeg -loglevel quiet -ss 26 -i 3196505-sd_960_540_30fps.mp4 -t 1  -f image2 - */
pub fn grab_wallpaper_image(wallpaper_video_path: &str) -> Result<String, FromUtf8Error>{
    let raw_image_output = Command::new("ffmpeg")
        .args(["-loglevel", "quiet", "-ss", "26", "-i", wallpaper_video_path, "-t", "1", "-f", "image2", "-"])
        .output()
        .expect("failed to grab image");

    return String::from_utf8(raw_image_output.stdout);
    
}

fn main(){
    let wallpaper_image_data = grab_wallpaper_image("/Users/coder/Wallpapers/3196505-sd_960_540_30fps.mp4");
}
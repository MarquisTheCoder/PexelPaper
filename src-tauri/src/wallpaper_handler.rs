
mod wallpaper;
use wallpaper::Wallpaper;
use std::process::{Command};

// I dont need to make this asynchronous I can just close and re run pids I over complicated the process

pub struct WallpaperHandler<'b>{
    current_wallpaper: &'b Wallpaper 
}

impl WallpaperHandler<'_>{
    
    pub fn play(wallpaper: Wallpaper){
        if !wallpaper.get_wallpaper_path().is_none() {
            match !wallpaper.get_wallpaper_path(){
                Some(wallpaper_path) =>{
                    const vlc_executable = "/Applications/VLC.app/Contents/MacOS/VLC";
                    const video_wallpaper_flag: &str = "--video-wallpaper";
                    const no_osd: &str = "--no-osd";
                    const loop_playback: &str = "-L";
                
                    let  mut run_wallpaper_in_background = Command::new(vlc_executable)
                        .arg(video_wallpaper_flag)
                        .arg(wallpaper_path)
                        .arg(loop_playback)
                        .arg(no_osd)
                        .spawn()
                        .expect("[-] Cannot run video in the background") 
                }
            }
        }
    }
}

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("path");
    WallpaperHandler::play(wallpaper);
}
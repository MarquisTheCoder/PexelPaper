
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
            match wallpaper.get_wallpaper_path(){
                Some(wallpaper_path) =>{
                    println!("making sure I'm getting the correct path: {}", wallpaper_path); 
                    
                    let  mut run_wallpaper_in_background = Command::new("/Applications/VLC.app/Contents/MacOS/VLC")
                        .arg("--video-wallpaper")
                        .arg(wallpaper_path)
                        .arg("-L")
                        .arg("--no-osd")
                        .spawn()
                        .expect("[-] Cannot run video in the background"); 
                },
                None => println!("Wallpaper path does not exist"),
                
            }
        }
    }
}

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("/Users/coder/ScreenCaptures/screen_recording.mov");
    WallpaperHandler::play(wallpaper);
}
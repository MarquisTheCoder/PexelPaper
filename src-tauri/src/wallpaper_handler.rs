
mod wallpaper;
use wallpaper::Wallpaper;
use std::process::{Command};

// I dont need to make this asynchronous I can just close and re run pids I over complicated the process

pub struct WallpaperHandler<'b>{
    old_wallpaper: &'b Wallpaper,
    current_wallpaper: &'b Wallpaper, 
    next_wallpapers: &'b Vec<Wallpaper> 
}

impl WallpaperHandler<'_>{
    
    pub fn new(wallpaper: Wallpaperr, next_wallpapers){
        WallpaperHandler{
            wallpapers: wallpaper,
        }
    }

    fn wallpaper_is_current() -> bool{
        Self::current_wallpaper == Self::old_wallpaper
    }

    fn the_wallpaper_exist(wallpaper: Wallpaper) -> bool{
        if wallpaper.get_wallpaper_path().is_none(){
            return false
        }
        return true
    }

   
    pub fn update_pid(wallpaper: Wallpaper, pid: u32){
        wallpaper.set_wallpaper_pid(pid);
    }

    pub fn play(wallpaper: Wallpaper){
        if Self::the_wallpaper_exist(wallpaper) {

            match wallpaper.get_wallpaper_path(){
                Some(wallpaper_path) =>{
                    println!("making sure I'm getting the correct path: {}", wallpaper_path); 
                    
                    let  mut run_wallpaper_in_background = Command::new("/Applications/VLC.app/Contents/MacOS/VLC")
                        .arg("--video-wallpaper")
                        .arg(wallpaper_path)
                        .arg("--noaudio")
                        .arg("-L")
                        .arg("--no-osd")
                        .spawn()
                        .expect("[-] Cannot run video in the background");
                   
                   //saving current vlc pid so we can close it and rerun it later
                   Self:;update_pid(wallpaper, run_wallpaper_in_background.id());
                },
                None => println!("Wallpaper path does not exist"), 
            }
        }else{

        }
    }
}

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("/Users/coder/Movies/testWallpaper.mp4");
    let wallpaper_handler: WallpaperHandler = WallpaperHandler::new(wallpaper, [wallpaper]);

}
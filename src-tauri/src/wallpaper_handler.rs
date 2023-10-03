
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

    fn the_wallpaper_is_current(wallpaper: Wallpaper) -> bool{
        match wallpaper.get_wallpaper_path(){
            Some(wallpaper_path) => {
                wallpaper_path == wallpaper.get_wallpaper_checksum_path()
            },
            None => {
                println!("The wallpaper does not exist");
                return false
            },
        }
    }

    fn the_wallpaper_exist(wallpaper: Wallpaper) -> bool{
        if wallpaper.get_wallpaper_path().is_none(){
            return false
        }
        return true
    }


    pub fn updateId(&mut self, pid: u32){
        self.current_wallpaper.set_wallpaper_pid(pid);
    }
    // :wpub fn updatePath(&mut self, path: &str){
    
    // }
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
                   wallpaper.set_wallpaper_pid(run_wallpaper_in_background.id()); 
                },
                None => println!("Wallpaper path does not exist"), 
            }
        }else{

        }
    }
}

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("/Users/coder/Movies/testWallpaper.mp4");
    WallpaperHandler::play(wallpaper);
}
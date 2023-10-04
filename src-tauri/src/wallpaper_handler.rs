
mod wallpaper;

use wallpaper::Wallpaper;
use std::process::{Command};

// I dont need to make this asynchronous I can just close and re run pids I over complicated the process

pub struct WallpaperHandler<'a >{
    current_wallpaper: &'a Wallpaper,
}

impl<'b> WallpaperHandler<'static>{

    pub fn kill_wallpaper(&self, pid: u32){
        const kill_command: &str = "kill";
        const flag_nine: &str = "-9";

        Command::new(kill_command)
            .arg(flag_nine)
            .arg(format!("{}", pid))
                .spawn()
                .expect("Could not kill the current process"); 
    }
    
    pub fn new<'c>(wallpaper: Wallpaper) -> WallpaperHandler<'b>{
        WallpaperHandler{
            current_wallpaper: &wallpaper,
        }
    }

    pub fn set_current_wallpaper(&mut self, wallpaper: Wallpaper){
        self.current_wallpaper = &wallpaper;
    }
    
    // pub fn get_current_wallpaper(&self) -> &Wallpaper{
    //     &self.current_wallpaper
        
    // }

    pub fn play(&mut self, mut wallpaper: Wallpaper) {
        if &wallpaper != self.current_wallpaper {
            match self.current_wallpaper.get_wallpaper_pid(){
                Some(current_wallpaper_pid) => self.kill_wallpaper(current_wallpaper_pid),
                None => { 
                    self.set_current_wallpaper(wallpaper);
                }, 
            };
        }else{
            println!("Playing the wallpaper...."); 
        }
        match &wallpaper.get_wallpaper_path(){
            Some(wallpaper_path) =>{
                println!("making sure I'm getting the correct path: {}", wallpaper_path); 
                
                let run_wallpaper_in_background = Command::new("/Applications/VLC.app/Contents/MacOS/VLC")
                    .arg("--video-wallpaper")
                    .arg(wallpaper_path)
                    .arg("--noaudio")
                    .arg("-L")
                    .arg("--no-osd")
                    .spawn()
                    .expect("[-] Cannot run video in the background");
                
                //saving current vlc pid so we can close it and rerun it later
                &wallpaper.set_wallpaper_pid(run_wallpaper_in_background.id());
            },
            None => println!("Wallpaper path does not exist"), 
        } 
    }

    

}

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("/Users/coder/Movies/testWallpaper.mp4");
    let wallpaper_handler: WallpaperHandler = WallpaperHandler::new(wallpaper);

}
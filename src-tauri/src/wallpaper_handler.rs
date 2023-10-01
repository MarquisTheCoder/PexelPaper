
mod wallpaper;
use wallpaper::Wallpaper;
use std::process::{Command};

// I dont need to make this asynchronous I can just close and re run pids I over complicated the process

pub struct WallpaperHandler<'b>{
    current_wallpaper: &'b Wallpaper 
}

impl WallpaperHandler<'_>{
    
    pub fn play(wallpaper: Wallpaper){
        if(!wallpaper.wallpaper_path.is_none()){
            println!("wallpaper path exist")
        }
    }
}

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("path");
}
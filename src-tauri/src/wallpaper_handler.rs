
mod wallpaper;
use wallpaper::Wallpaper;

// I dont need to make this asynchronous I can just close and re run pids I over complicated the process

pub struct WallpaperHandler<'b>{
    current_wallpaper: &'b Wallpaper 
}

impl WallpaperHandler<'_>{

}

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("path");
}
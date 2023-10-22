
mod wallpaper;

use wallpaper::Wallpaper;

// I dont need to make this asynchronous I can just close and re run pids I over complicated the process

pub struct WallpaperHandler<'a >{
    current_wallpaper: &'a Wallpaper,
}

impl<'b> WallpaperHandler<'static>{
    
}

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("/Users/coder/Movies/testWallpaper.mp4");
    let wallpaper_handler: WallpaperHandler = WallpaperHandler::new(wallpaper);
}
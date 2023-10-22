
mod wallpaper;

use wallpaper::Wallpaper;
// I dont need to make this asynchronous I can just close and re run pids I over complicated the process

pub struct WallpaperHandler<'a >{
    pub current_wallpaper: &'a mut Wallpaper,
}

impl<'b> WallpaperHandler<'static>{

    pub fn new(wallpaper: &Wallpaper) -> WallpaperHandler<'b>{
        WallpaperHandler{
            current_wallpaper: wallpaper,
        }
    }

    fn check_current_wallpaper_active(&mut self) -> bool{
        match self.current_wallpaper.get_wallpaper_pid(){
            Some(_empty) => true,
            None => false
        }
    }

    fn kill_current_wallpaper(&mut self){
        self.current_wallpaper.kill();
    }

    pub fn set_current_wallpaper(&mut self, wallpaper: Wallpaper){
        
        if self.check_current_wallpaper_active(){
            self.kill_current_wallpaper();
        };
        self.current_wallpaper = wallpaper.clone();
    }

    pub fn run_current_wallpaper(&self){
        self.current_wallpaper.play();
    }
}

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("/Users/coder/Movies/testWallpaper.mp4");
    // let wallpaper_handler: WallpaperHandler = WallpaperHandler::new(wallpaper);
}
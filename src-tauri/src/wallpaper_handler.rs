
mod wallpaper;

use std::{thread, time};
use wallpaper::Wallpaper;
// I dont need to make this asynchronous I can just close and re run pids I over complicated the process

pub struct WallpaperHandler<'a >{
    pub current_wallpaper: &'a mut Wallpaper,
}

impl<'b> WallpaperHandler<'_>{

    fn check_current_wallpaper_active(&mut self) -> bool{
        match self.current_wallpaper.get_wallpaper_pid(){
            Some(_empty) => true,
            None => false
        }
    }

    fn play_current_wallpaper(&mut self){
        self.current_wallpaper.play()
    }
    
    // static EMPTY_WALLPAPER: Wallpaper = Wallpaper::new("default");
    pub fn new(wallpaper: &'b mut Wallpaper) -> WallpaperHandler<'b>{
        WallpaperHandler{
            current_wallpaper: wallpaper,
        }
    }

    //proud of it
    pub fn print_current_wallpaper(&self){
        match self.current_wallpaper.get_wallpaper_path(){
            Some(wallpaper_path) => {
                println!("{}", wallpaper_path);
            },
            None => {
                println!("Nothings happening");
            }
        }
    }

    pub fn kill_current_wallpaper(&mut self){
        self.current_wallpaper.kill();
    }

    pub fn change_current_wallpaper(&mut self, wallpaper: & mut Wallpaper){
        
        if self.check_current_wallpaper_active(){
            self.kill_current_wallpaper();
        }
        *self.current_wallpaper =  wallpaper.clone();
        self.play_current_wallpaper();
    }

}

fn main(){
    let mut _wallpaper:Wallpaper = Wallpaper::new("/Users/coder/Movies/testWallpaper.mp4");
    let mut wallpaper2: Wallpaper = Wallpaper::new("/Users/coder/Movies/peaceful_vroom.mp4"); 
    let mut wallpaper_handler: WallpaperHandler = WallpaperHandler::new(&mut _wallpaper);
    wallpaper_handler.print_current_wallpaper();
    wallpaper_handler.play_current_wallpaper();

    let five_seconds = time::Duration::from_millis(10000);
    println!("waiting five seconds");
    thread::sleep(five_seconds);
    wallpaper_handler.change_current_wallpaper(&mut wallpaper2);
    // println!("{}",  wallpaper_handler.check_current_wallpaper_active());
}
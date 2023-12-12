
mod wallpaper;

use std::{thread, time};
use wallpaper::Wallpaper;
// I dont need to make this asynchronous I can just close and re run pids I over complicated the process

pub struct WallpaperHandler{
    pub current_wallpaper:  Box<Wallpaper>,
}

impl WallpaperHandler{

    fn check_current_wallpaper_active(&mut self) -> bool{
        match self.current_wallpaper.get_wallpaper_pid(){
            Some(_empty) => true,
            None => false
        }
    }

    fn play_current_wallpaper(&mut self){
        let mut current_wallpaper: Wallpaper =  *self.current_wallpaper.clone();  
       current_wallpaper.play();
    }
    
    pub fn new(wallpaper: & str) -> WallpaperHandler {
        let _wallpaper = Box::new(Wallpaper::new(wallpaper));    
        WallpaperHandler {
            current_wallpaper: _wallpaper, // Assigns a mutable reference to current_wallpaper
        }
    }
     

    pub fn kill_current_wallpaper(&mut self){
        self.current_wallpaper.kill();
    }

    pub fn change_current_wallpaper(&mut self, wallpaper: &str){
        
        let mut _wallpaper:Wallpaper = Wallpaper::new(wallpaper);
        if self.check_current_wallpaper_active(){
            self.kill_current_wallpaper();
        }
        self.current_wallpaper =  Box::new(_wallpaper.clone());
        self.play_current_wallpaper();
    }
}

fn main(){
    
    let mut wallpaper_handler: WallpaperHandler = WallpaperHandler::new("");
    wallpaper_handler.play_current_wallpaper();

    // println!("{}",  wallpaper_handler.check_current_wallpaper_active());
}
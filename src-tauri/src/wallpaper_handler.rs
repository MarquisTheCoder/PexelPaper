
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
    pub fn new(wallpaper: &'b str) -> WallpaperHandler<'b>{
        let _wallpaper:Wallpaper = &mut  Wallpaper::new(wallpaper);
        WallpaperHandler{
            current_wallpaper: _wallpaper,
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

    pub fn change_current_wallpaper(&mut self, wallpaper: &str){
        
        let mut _wallpaper:Wallpaper = Wallpaper::new(wallpaper);
        if self.check_current_wallpaper_active(){
            self.kill_current_wallpaper();
        }
        *self.current_wallpaper =  _wallpaper.clone();
        self.play_current_wallpaper();
    }
}

fn main(){
    
    let mut wallpaper_handler: WallpaperHandler = WallpaperHandler::new("");
    wallpaper_handler.print_current_wallpaper();
    wallpaper_handler.play_current_wallpaper();

    // println!("{}",  wallpaper_handler.check_current_wallpaper_active());
}
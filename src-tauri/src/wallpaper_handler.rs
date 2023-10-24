
mod wallpaper;

use wallpaper::Wallpaper;
// I dont need to make this asynchronous I can just close and re run pids I over complicated the process

pub struct WallpaperHandler<'a >{
    pub current_wallpaper: &'a mut Wallpaper,
}

impl<'b> WallpaperHandler<'_>{
    static EMPTY_WALLPAPER: mut Wallpaper = Wallpaper::new("default");
    pub fn new(wallpaper: &'b mut Wallpaper) -> WallpaperHandler<'b>{
        WallpaperHandler{
            current_wallpaper: wallpaper,
        }
    }

    //proud of it
    fn print_wallpaper(&self){
        match self.current_wallpaper.get_wallpaper_path(){
            Some(wallpaper_path) => {
                println!("{}", wallpaper_path);
            },
            None => {
                println!("Nothings happening");
            }
        }
    }

    fn check_current_wallpaper_active(&mut self) -> bool{
        match self.current_wallpaper.get_wallpaper_pid(){
            Some(_empty) => true,
            None => false
        }
    }

    pub fn kill_current_wallpaper(&mut self){
        self.current_wallpaper.kill();
    }

    pub fn play_current_wallpaper(&mut self){
       self.current_wallpaper.play()
    }

    pub fn set_current_wallpaper(&mut self, wallpaper: & mut Wallpaper){
        
        if self.check_current_wallpaper_active(){
            self.kill_current_wallpaper();
        };
        let mut copy = Wallpaper::new("");
        match wallpaper.get_wallpaper_path(){
            Some(wallpaper_path) => {
                copy.set_wallpaper_path(&wallpaper_path);
            },
            None => {
                println!("do nothing");
            }
        }

    }

}

fn main(){
    let mut _wallpaper:Wallpaper = Wallpaper::new("/Users/coder/Movies/testWallpaper.mp4");
    let mut wallpaper_handler: WallpaperHandler = WallpaperHandler::new(&mut _wallpaper);
    wallpaper_handler.print_wallpaper();
    wallpaper_handler.play_current_wallpaper();
    println!("{}",  wallpaper_handler.check_current_wallpaper_active());
}

mod wallpaper;

use wallpaper::Wallpaper;
// I dont need to make this asynchronous I can just close and re run pids I over complicated the process

pub struct WallpaperHandler<'a >{
    pub current_wallpaper: &'a mut Wallpaper,
}

impl<'b> WallpaperHandler<'static>{

    pub fn new(wallpaper: &'b mut Wallpaper) -> WallpaperHandler<'b>{
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

    pub fn play_current_wallpaper(&mut self){
       let wallpaper: Wallpaper = self.current_wallpaper.clone();
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

        match wallpaper.get_wallpaper_pid(){
            Some(wallpaper_pid) => {
                copy.set_wallpaper_pid(wallpaper_pid);
            },
            None => {
                println!("do nothing");
            }
        }
        
        *self.current_wallpaper = copy;
    }

}

fn main(){
    let  _wallpaper:&mut Wallpaper = &mut Wallpaper::new("/Users/coder/Movies/testWallpaper.mp4");
    let wallpaper_handler: WallpaperHandler = WallpaperHandler::new(_wallpaper);
    wallpaper_handler.play_current_wallpaper();
}
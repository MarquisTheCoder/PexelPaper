
pub struct Wallpaper<'a>{
    wallpaper_path: Option<String>,
    wallpaper_pid: Option<i16> 
}


impl Wallpaper<'_>{

    fn new(path: &str, pid: i16) -> Self {
        Wallpaper {
            wallpaper_path: Some(path.to_string()),
            wallpaper_pid: Some(pid)
        }
    }
    
    fn get_wallpaper_path(&self) -> Option<String>{
        self.wallpaper_path.clone()
    }

    fn get_wallpaper_pid(&self) -> Option<i16>{
        self.wallpaper_pid.clone()
    }

    fn set_wallpaper_path(&mut self, path: &str){
        self.wallpaper_path = Some(path.to_string());
    } 

    fn set_wallpaper_id(&mut self, pid: i16){
        self.wallpaper_pid = Some(pid); 
    }
}

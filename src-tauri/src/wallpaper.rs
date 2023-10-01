
pub struct Wallpaper{
    wallpaper_path: Option<String>,
    wallpaper_pid: Option<i16> 
}

impl Wallpaper{

    pub fn new(path: &str) -> Self {
        Wallpaper {
            wallpaper_path: Some(path.to_string()),
            wallpaper_pid: None
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

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("path");
    match wallpaper.get_wallpaper_path(){
        Some(wallpaper_path) => println!("Wallpaper path is: {}", wallpaper_path),
        None => println!("Wallpaper path is empty"),
    }
}
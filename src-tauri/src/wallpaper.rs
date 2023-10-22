
#[derive(PartialEq)]
pub struct Wallpaper{
    wallpaper_path: Option<String>,
    wallpaper_pid: Option<u32>, 
}

impl Wallpaper{

    pub fn new(path: &str) -> Self {
        Wallpaper {
            wallpaper_path: Some(path.to_string()),
            wallpaper_pid: None
        }
    }

    pub fn clone(&self) -> Wallpaper{
        let clone_wallpaper = 
    }

    pub fn get_wallpaper_path(&self) -> Option<String>{
        self.wallpaper_path.clone()
    }

    pub fn get_wallpaper_pid(&self) -> Option<u32>{
        self.wallpaper_pid.clone()
    }

    pub fn set_wallpaper_path(&mut self, path: &str){
        self.wallpaper_path = Some(path.to_string());
    } 

    pub fn set_wallpaper_pid(&mut self, pid: u32){
        self.wallpaper_pid = Some(pid); 
    }
    pub fn play_wallpaper(mut self){

        const vlc_executable = "/Applications/VLC.app/Contents/MacOS/VLC";
        const video_wallpaper: &str = "--video-wallpaper";
        const no_osd: &str = "--no-osd";
        const loop_playback: &str = "-L";
        let video_path: &str =  "";

        let mut run_wallpaper_in_background = Command::new(vlc_executable)
            .arg(video_wallpaper)
            .arg(video_path)
            .arg(loop_playback)
            .arg(no_osd)
                .spawn()
                .expect("[-] Cannot run video in the background");

        self.set_wallpaper_pid(run_wallpaper_in_background.id());
    }
    pub fn stop_wallpaper(mut self){
       let stopping_wallpaper = Command::new();
    }
}

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("path");
    match wallpaper.get_wallpaper_path(){
        Some(wallpaper_path) => println!("Wallpaper path is: {}", wallpaper_path),
        None => println!("Wallpaper path is empty"),
    }
}
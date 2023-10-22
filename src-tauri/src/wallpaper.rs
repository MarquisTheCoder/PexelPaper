
use std::process::{Command};
use std::{thread, time};

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

    pub fn play(&mut self){

        const VLC_EXECUTABLE: &str = "/Applications/VLC.app/Contents/MacOS/VLC";
        const VIDEO_WALLPAPER: &str = "--video-wallpaper";
        const NO_AUDIO: &str = "--noaudio";
        const NO_OSD: &str = "--no-osd";
        const LOOP_PLAYBACK: &str = "-L";

        match self.get_wallpaper_path(){
            Some(wallpaper_path) =>{
                println!("making sure I'm getting the correct path: {}", wallpaper_path); 
                
                let run_wallpaper_in_background = Command::new(VLC_EXECUTABLE)
                    .arg(VIDEO_WALLPAPER)
                    .arg(wallpaper_path)
                    .arg(NO_AUDIO)
                    .arg(LOOP_PLAYBACK)
                    .arg(NO_OSD)
                    .spawn()
                    .expect("[-] Cannot run video in the background");
                
                //saving current vlc pid so we can close it and rerun it later
                self.set_wallpaper_pid(run_wallpaper_in_background.id());
            },
            None => {
                println!("cannot display wallpaper");
            }
        }

    }

    pub fn kill(&mut self){

        const KILL_COMMAND: &str = "kill";
        const FLAG_NINE: &str = "-9";

        match self.get_wallpaper_pid(){
            Some(wallpaper_pid){
                Command::new(KILL_COMMAND)
                    .arg(FLAG_NINE)
                    .arg(format!("{}", wallpaper_pid))
                        .spawn()
                        .expect("Could not kill the current process")
            },
            None =>{
                prinln!("wallpaper has no PID");
            }
        }
        
    }
}

fn main(){
    let mut wallpaper1: Wallpaper = Wallpaper::new("/Users/coder/Movies/peaceful_vroom.mp4");
    match wallpaper.get_wallpaper_path(){
        Some(wallpaper_path) => println!("Wallpaper path is: {}", wallpaper_path),
        None => println!("Wallpaper path is empty"),
    }
    println!("playing the wallpaper now");
   wallpaper1.play();
   let five_seconds = time::Duration::from_seconds(5);
   thread::sleep(five_seconds);
   wallpaper1.kill();

}
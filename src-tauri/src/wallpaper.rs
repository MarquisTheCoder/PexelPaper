
use std::process::{Command};

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

    pub fn play_wallpaper(&mut self){

        const VLC_EXECUTABLE: &str = "/Applications/VLC.app/Contents/MacOS/VLC";
        const VIDEO_WALLPAPER: &str = "--video-wallpaper";
        const NO_AUDIO: &str = "--noaudio";
        const NO_OSD: &str = "--no-osd";
        const LOOP_PLAYBACK: &str = "-L";
        let video_path: &str =  "";

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

    pub fn kill_wallpapr(&self, pid: u32){
        const KILL_COMMAND: &str = "kill";
        const FLAG_NINE: &str = "-9";

        Command::new(KILL_COMMAND)
            .arg(FLAG_NINE)
            .arg(format!("{}", pid))
                .spawn()
                .expect("Could not kill the current process");
    }
}

fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("path");
    match wallpaper.get_wallpaper_path(){
        Some(wallpaper_path) => println!("Wallpaper path is: {}", wallpaper_path),
        None => println!("Wallpaper path is empty"),
    }
}
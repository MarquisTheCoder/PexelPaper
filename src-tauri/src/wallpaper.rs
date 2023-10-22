
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

    pub fn play_wallpaper(mut self){

        const VLC_EXECUTABLE: &str = "/Applications/VLC.app/Contents/MacOS/VLC";
        const VIDEO_WALLPAPER: &str = "--video-wallpaper";
        const NO_OSD: &str = "--no-osd";
        const LOOP_PLAYBACK: &str = "-L";
        let video_path: &str =  "";

        const run_wallpaper_in_background = Command::new(VLC_EXECUTABLE)
            .arg(VIDEO_WALLPAPER)
            .arg(video_path)
            .arg(LOOP_PLAYBACK)
            .arg(NO_OSD)
                .spawn()
                .expect("[-] Cannot run video in the background");

        self.set_wallpaper_pid(run_wallpaper_in_background.id());
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
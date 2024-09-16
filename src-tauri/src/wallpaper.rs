
use std::process::{Command};
use std::{thread, time};

#[derive(PartialEq, Clone)]
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

    pub fn equals(&self, other: &Wallpaper) -> bool{
        if self.get_wallpaper_path().is_some() && other.get_wallpaper_path().is_some() {
            let current_wallpaper_path: String = self.get_wallpaper_path().unwrap();
            let other_wallpaper_path: String = other.get_wallpaper_path().unwrap();

            return current_wallpaper_path == other_wallpaper_path;
        }
        return false;
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
        // const MPV_EXECUTABLE: &str = "mpv";
        const NO_CONFIG: &str = "--no-config";
        const NO_INPUT_CURSOR: &str = " --no-input-cursor";
        const VIDEO_WALLPAPER: &str = "--video-wallpaper";
        const NO_AUDIO: &str = "--no-audio";
        const NO_NATIVE_FS: &str = " --no-native-fs";
        // const NO_OSD: &str = "--no-osd-bar";
        const QUIET: &str = "--really-quiet";
        // const RUN_IN_BG: &str = "&";
        match self.get_wallpaper_path(){
            Some(wallpaper_path) =>     {
                println!("making sure I'm getting the correct path: {}", wallpaper_path);

                let run_wallpaper_in_background = Command::new(VLC_EXECUTABLE)
                    .arg(wallpaper_path)
                    .arg(VIDEO_WALLPAPER)
                    // .arg(NO_OSD)
                    .arg(NO_AUDIO)
                    // .arg(NO_CONFIG)
                    // .arg(QUIET)
                    // .arg(NO_INPUT_CURSOR)
                    // .arg(NO_NATIVE_FS)
                    // .arg(RUN_IN_BG)
                    .spawn()
                    .expect("[-] Cannot run video in the background");

               
                //saving current vlc pid so we can close it and rerun it later
                // self.set_wallpaper_pid(run_wallpaper_in_background.id());
            },
            None => {
                println!("cannot display wallpaper")
            }
        }
    }

    pub fn kill(&mut self){
        // const KILL_COMMAND: &str = "kill";
        // const FLAG_NINE: &str = "-9";
        // loop{
        // match self.get_wallpaper_pid(){
        //     Some(wallpaper_pid) => {

        //         Command::new(KILL_COMMAND)
        //             .arg(FLAG_NINE)
        //             .arg(format!("{}", wallpaper_pid))
        //                 .spawn()
        //                 .expect("Could not kill the current process");
        //     },
        //     None =>{
        //         println!("wallpaper has no PID");
        //     }
        // }
        let ps_output = Command::new("pgrep")
            .arg("-f")
            .arg("/Applications/VLC.app/Contents/MacOS/VLC")
            .output()
            .expect("Failed to execute ps");
        let ps_output_str = String::from_utf8(ps_output.stdout).expect("Failed to convert ps output to string");

        for line in ps_output_str.lines() {
       
            let pid: u32 = line.trim().split_whitespace().next().unwrap().parse().unwrap();
            println!("Found VLC PID: {}", pid);
            self.set_wallpaper_pid(pid);
            let kill_command = Command::new("kill")
                .arg("-9")
                .arg(format!("{}", pid))
                .spawn()
                .expect("could not kill process");
            
        }
    }
}
fn main(){
    //test
    let mut wallpaper1: Wallpaper = Wallpaper::new("/Users/coder/Wallpapers/ele.mp4:w
");
   wallpaper1.play(); 
    //testing equals
    
}
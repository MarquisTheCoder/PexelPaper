
use std::process::{Command};
use std::{thread, time};

#[derive(PartialEq, Clone)]
pub struct Wallpaper{
    wallpaper_path: Option<String>,
    wallpaper_pid: Option<u32>,
}

impl Wallpaper{

    pub fn play(path: &str){
        const VLC_EXECUTABLE: &str = "/Applications/VLC.app/Contents/MacOS/VLC";
        const NO_CONFIG: &str = "--no-config";
        const NO_INPUT_CURSOR: &str = " --no-input-cursor";
        const VIDEO_WALLPAPER: &str = "--video-wallpaper";
        const NO_AUDIO: &str = "--no-audio";
        const NO_NATIVE_FS: &str = " --no-native-fs";
        // const NO_OSD: &str = "--no-osd-bar";
        const QUIET: &str = "--really-quiet";
        // const RUN_IN_BG: &str = "&";

        let run_wallpaper_in_background = Command::new(VLC_EXECUTABLE)
            .arg(path)
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
           
        
    }

    pub fn kill(){
       
        let ps_output = Command::new("pgrep")
            .arg("-f")
            .arg("/Applications/VLC.app/Contents/MacOS/VLC")
            .output()
            .expect("Failed to execute ps");

        let ps_output_str = String::from_utf8(ps_output.stdout).expect("Failed to convert ps output to string");

        for line in ps_output_str.lines() {
       
            let pid: u32 = line.trim().split_whitespace().next().unwrap().parse().unwrap();
            println!("Found VLC PID: {}", pid);
            let kill_command = Command::new("kill")
                .arg("-9")
                .arg(format!("{}", pid))
                .spawn()
                .expect("could not kill process"); 
        }
    }
}

fn main(){
  
   Wallpaper::play("/Users/coder/Wallpapers/ele.mp4");
   let five_seconds = time::Duration::from_millis(20000);
   println!("waiting five seconds");
   thread::sleep(five_seconds);
   Wallpaper::kill(); 
    //testing equals
    
}
use std::process::{Command};




fn main() {

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
        
    // Get the id of the spawned process so we can kill it later.
    let process_id = run_wallpaper_in_background.id();

    
    println!("Spawned process with process_id: {}", process_id);


    // You can also wait for the process to finish, if needed.
    let status = cmd.wait().expect("Failed to wait for command");

    println!("Command exited with: {:?}", status);
}

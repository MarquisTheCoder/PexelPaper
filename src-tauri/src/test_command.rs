use std::process::{Command};



fn main() {

    let arg_current_vid: &str = ""; 
    let com_vlc: &str = "/Applications/VLC.app/Contents/MacOS/VLC";
    let full_command: &str = format!("{} --video-wallpaper {} -L", com_vlc, arg_current_vid );

    // Specify the command you want to run.
    let  mut cmd = Command::new(full_command)
        .arg("-l")
        .spawn()
        .expect("Failed to start command");

    // Get the PID of the spawned process.
    let pid = cmd.id();

    println!("Spawned process with PID: {}", pid);

    // You can also wait for the process to finish, if needed.
    let status = cmd.wait().expect("Failed to wait for command");
    println!("Command exited with: {:?}", status);
}



extern crate base64;

use core::str;
use std::process::{Command, Stdio};
use base64::encode;

/*ffmpeg -loglevel quiet -ss 26 -i 3196505-sd_960_540_30fps.mp4 -t 1  -f image2 - */

pub fn raw_data_to_base64(wallpaper_video_path: &str) -> String{
    // ╰─  ffmpeg -loglevel quiet -ss 26 -i ele.mp4 -t 1 -f image2 - |od -vt x1|awk '// Run ffmpeg to extract a frame from the video
    let image_data = Command::new("ffmpeg")
    .args(["-loglevel", "quiet", "-ss", "26", "-i", wallpaper_video_path, "-t", "1", "-f", "image2", "-"])
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

// Run od to convert the binary output into hex
let od = Command::new("od")
    .args(&["-vt x1"])
    .stdin(Stdio::from(image_data.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()  
    .unwrap();

// Run awk to process the od output
let awk = Command::new("awk")
    .args(&["{ $1=\"\"; print }"])
    .stdin(Stdio::from(od.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

    
// Capture the output of awk
    let output = awk.wait_with_output().unwrap();

    // Check if output is empty
    if output.stdout.is_empty() {
        eprintln!("No output from awk");
        return String::new();
    }
// Convert the output to a string, using lossy conversion to avoid errors with non-UTF-8 bytes
    let result = String::from_utf8_lossy(&output.stdout);

    println!("result: {}", &result);
    
    return String::from(result);
     
}
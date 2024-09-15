

extern crate base64;

use std::process::{Command, Stdio};
use base64::encode;

/*ffmpeg -loglevel quiet -ss 26 -i 3196505-sd_960_540_30fps.mp4 -t 1  -f image2 - */

pub fn raw_data_to_base64(wallpaper_video_path: &str) -> String{
    // ╰─  ffmpeg -loglevel quiet -ss 26 -i ele.mp4 -t 1 -f image2 - |od -vt x1|awk '{$1="";print}';
    let image_data = Command::new("ffmpeg")
        .args(["-loglevel", "quiet", "-ss", "26", "-i", &wallpaper_video_path, "-t", "1", "-f", "image2", "-"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // od -vt x1|awk '{$1="";print}';
    let od_to_hex= Command::new("od")
        .args(["-vt", "x1"])
        .stdin(Stdio::from(image_data.stdout.unwrap()))
        .stdout(Stdio::piped());

    let awk = Command::new("awk")
        .args("\'{$1=\"\";print}\'")
        .stdin()
        .stdout()
        .unwrap();

    let stdout: String = String::from_utf8(bytes.stdout).unwrap();

    println!("{}", &stdout);
    

//     let hex : String = bytes.stdout.iter()
//   .map(|b| format!("{:02x}", b).to_string())
//   .collect::<Vec<String>>()
//   .join(" ");

     // Encode the binary image data as base64
     
     println!("base64_image: ", );
     return stdout;
      
}


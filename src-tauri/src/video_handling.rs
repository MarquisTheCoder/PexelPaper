
// use std::fs;
use std::path::Path;
use async_std::sync::channel;
use async_std::task;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/*
    this area of the program is responsible for
    managing video files and keeping track of the current 
    video being played by PexelPaper
*/

pub struct VideoHandler<'a>{
    pub root: &str,
    pub current_video: Arc<Mutex<&'a str>>,
}

impl VideoHandler{ 
    
    pub fn new(root: &str, current_video: &str) -> Self{
        Self {
            root: root,
            current_video: current_video
        }
    }

    //basicially an event listener on the current video variable 
    //thatll let me dynamically load video data
    async fn listen_for_current_video_changes(&self) { 
        let (sender, receiver) = channel(1); // Create a channel with a buffer of 1

        let data = Arc::clone(&self.current_video);
        let mut previous_video_path= self.current_video.lock().clone(); 

        task::spawn(async move {
            loop {
                {
                    let mut video_path_guard = self.current_video.lock();
                    if *video_path_guard != previous_video_path{
                        // Check if the data has changed
                        previous_video_path = video_path_guard.clone(); // update previous value
                        sender.send(()).await.expect("Send failed");
                    }
                }
                task::sleep(Duration::from_secs(1)).await;
            }
        });

        while let Some(_) = receiver.recv().await {
            println!("Video path has changed: {}", *self.current_video.lock());
        }
    } 

    fn check_video_exist(video_path: &str) -> bool{
        let path = Path::new(video_path);
        path.exists() && path.is_file()
    }

    pub fn change_root(&self, new_root: &str) { //returns void
        self.root = new_root;
    }
    
    pub fn change_current_video(&self, new_selection: &str) { //return void
        self.current_video = new_selection;
    }

}

fn main(){

}


use std::fs;
use std::path::Path;
use async_std::sync::channel;
use async_std::task;
use std::sync::{Arc, Mutex};
use std::time::Duration;


pub struct VideoHandler{
    pub root: String,
    pub current_video: Arc<Mutex<String>>,
}

impl VideoHandler{ 

    pub fn new(root: &str, current_video: &str) -> Self{
        Self {
            root: root,
            current_video: current_video
        }
    }
    
    async fn listen_for_current_video_changes(&self) {
        let (sender, receiver) = channel(1); // Create a channel with a buffer of 1

        let data = Arc::clone(&self.current_video);
        let mut previous_video_path= self.current_video.lock().await.clone(); 

        task::spawn(async move {
            loop {
                {
                    let mut video_path_guard = current_video.lock().await;
                    if *video_path_guard != previous_video_path{
                        // Check if the data has changed
                        previous_value = data_guard.clone(); // update previous value
                        sender.send(()).await.expect("Send failed");
                    }
                }
                task::sleep(Duration::from_secs(1)).await;
            }
        });

        while let Some(_) = receiver.recv().await {
            println!("Video path has changed: {}", *self.current_video.lock().await);
        }
    } 

    fn check_video_exist(video_path: &str) -> bool{
        let path = Path::new(video_path);
        path.exists() && path.is_file()
    }

    pub fn check_video_defined(video_path: &str) -> bool{ 
            true; 
    }
    pub fn change_root(new_root: &str) { //returns void
        self.root = new_root;
    }
    
    pub fn change_current_video(video_selection: &str) { //return void
        self.current_video = video_selection;
    }
}
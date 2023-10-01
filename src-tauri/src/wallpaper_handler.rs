
mod wallpaper;
use wallpaper::Wallpaper;

// I dont need to make this asynchronous I can just close and re run pids I over complicated the process
//useful algorithms I may use to do this

/*
    Sliding Window Algorithm:
        Use Case: When you need to maintain a window of the most recent data points (a subset of the entire data) and perform operations on that window.
        Application: Calculate rolling statistics (e.g., rolling average, rolling sum), detect patterns or anomalies in a continuous data stream, or process recent data within a moving time window.
        Example: If you're monitoring temperature data, you can use a sliding window to calculate the average temperature over the last hour, updating it as new temperature readings arrive.

    Circular Buffers (Ring Buffers):
        Use Case: When you want to efficiently manage and maintain a fixed-size buffer for continuously incoming data.
        Application: Store incoming data in a circular buffer and process or retrieve the most recent data from the buffer.
        Example: Buffering audio samples in real-time audio processing or storing incoming network packets for processing.

    FIFO Queue (First-In-First-Out):
        Use Case: When you have a continuous stream of items, and you need to process them in the order they arrived.
        Application: Manage a queue of tasks, jobs, or requests, ensuring they are processed in the order they were received.
        Example: Handling incoming requests in a web server, processing tasks in a task scheduler, or managing a message queue.
*/

// pub struct WallpaperHandler<'b>{
//     current_wallpaper: &'b Wallpaper 
// }

// impl WallpaperHandler<'_>{


fn main(){
    let wallpaper: Wallpaper = Wallpaper::new("path");
}
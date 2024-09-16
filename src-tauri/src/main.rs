// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate whoami;
extern crate tauri;

mod grab_wallpaper_image;
mod wallpaper_handler;

use whoami::username;
use grab_wallpaper_image::raw_data_to_base64;
use wallpaper_handler::WallpaperHandler;

#[tauri::command]
fn current_user() -> String{
  return username();
}



#[tauri::command]
fn get_base64(path: &str) -> String{
  return raw_data_to_base64(path);
}
fn main() {

  let handler: WallpaperHandler = WallpaperHandler::new(); 
  
  raw_data_to_base64("/Users/coder/Wallpapers
/ele.mp4");
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![current_user])
    .invoke_handler(tauri::generate_handler![get_base64])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
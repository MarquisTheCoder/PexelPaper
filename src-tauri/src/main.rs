// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate whoami;
mod grab_wallpaper_image;

use whoami::username;
use grab_wallpaper_image::raw_data_to_base64;


#[tauri::command]
fn current_user() -> String{
  return username();
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![current_user])
    .invoke_handler(tauri::generate_handler![raw_data_to_base64])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
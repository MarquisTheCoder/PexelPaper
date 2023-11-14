// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate whoami;

use whoami::username;

#[tauri::command]
fn folder_to_svelte_store(folder_path: String){

}

#[tauri::command]
fn current_user() -> String{
  return username();
}

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
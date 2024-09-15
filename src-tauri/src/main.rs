// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate whoami;
extern crate base64;

use whoami::username;

use base64::{encode};

#[tauri::command]
fn current_user() -> String{
  return username();
}

fn main() {
  let data: Vec<u8> = vec![1,2,3,4,5];
    println!("{}", encode(&data));
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![current_user])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
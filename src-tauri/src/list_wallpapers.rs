use std::io;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::{self, DirEntry, File, metadata};
use std::vec;

pub fn list_wallpapers(folder_path_string: &str){
    let vital_file_extensions = [
        ".mov",
        ".mp4",
        ".avif",
    ];
    let found_wallpapers: &mut Vec<String>;

    let folder_path: &Path = Path::new(folder_path_string);
    
    if metadata(&folder_path)?.is_dir(){

        let paths = fs::read_dir(&folder_path);

        for path_result in paths{

            let full_path: PathBuf = path_result?.path();
            let file_name: &OsStr = full_path.file_name().unwrap();
            let file_name_str: &str = file_name.to_str().unwrap();

            // if !metadata(&full_path)?.is_dir(){
                
            // }
        }

    }
}

fn main(){
    list_wallpapers("/usr/local/codium/projects/PexelPaper/src-tauri/src");
}
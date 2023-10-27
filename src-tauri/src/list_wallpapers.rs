use std::io;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs::{self, DirEntry, File, metadata};
use std::vec;

pub fn listFiles(){
    let vital_file_extensions = [
        ".mov",
        ".mp4",
        ".avif",
    ];
    
}
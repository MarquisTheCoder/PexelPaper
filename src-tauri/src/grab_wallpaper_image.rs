
use std::path::Path;
use vid2img::FileSource;

pub fn grab_wallpaper_image(wallpaper_image_path: &str) -> &'static str{
    let file_path = Path::new(wallpaper_image_path);
    let frame_source = FileSource::new(file_path, (200, 200)).unwrap();
    let png_img_data = frame_source.into_iter().next(); 
    if Ok(Some(png_img_data)){
       return "data succeeded";
    }

    return "data failed";
    
}

fn main(){

}
use image::{ImageReader, DynamicImage, ImageError};

use crate::application::resource_loader::path_navigator;


pub fn load_image(image_directory_relative_path: String, image_file_relative_path: String) -> Result<DynamicImage, ImageError> {
    let path = 
        path_navigator::get_image_directory_path(image_directory_relative_path) + "/" +
        image_file_relative_path.as_str();
        
    ImageReader::open(path)?.decode()
}
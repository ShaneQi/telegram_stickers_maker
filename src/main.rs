extern crate image;

use std::env;
use std::path::Path;
use std::fs::{self, File};

fn main() {
    let mut working_path = env::args().nth(1).unwrap_or("./".to_string());
    if working_path.chars().rev().nth(0).unwrap() != '/' {
        working_path += "/";
    }

    let _ = fs::create_dir_all(&format!("{}/telegram_stickers/", working_path));

    for dir_item in Path::new(&format!("{}", working_path)).read_dir().expect(
        &format!(
            "Failed to find post directory: {}",
            working_path
        ),
    )
    {
        if let Ok(file_path) = dir_item.map(|i| i.path()) {
            if let Some(image) = image::open(file_path.clone()).ok() {
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                let mut new_file = File::create(&format!(
                    "{}/telegram_stickers/{}.png",
                    working_path,
                    file_name
                )).expect("");
                let resized_image = image.resize(512, 512, image::FilterType::Nearest);
                match resized_image.save(&mut new_file, image::ImageFormat::PNG) {
                    Ok(_) => println!("Succeeded to resized {}", file_name),
                    Err(_) => println!("Failed to resized {}", file_name),
                }
            }
        }
    }


}
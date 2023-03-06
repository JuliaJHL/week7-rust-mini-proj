extern crate image;
extern crate walkdir;
extern crate minifb;

use std::path::Path;
use std::env;
use walkdir::WalkDir;

use std::io::{self, Write};
use image::{DynamicImage, imageops,GenericImageView};
use minifb::{Key, Window, WindowOptions};

fn main() {
    // Get the directory path from the command-line arguments
    let args: Vec<String> = env::args().collect();
    let dir_path = match args.get(1) {
        Some(path) => Path::new(path),
        None => {
            println!("Usage: cargo run <directory_path>");
            return;
        }
    };

    // Collect all the image files in the directory and its subdirectories
    let mut image_files = Vec::new();
    for entry in WalkDir::new(dir_path) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.is_file() {
            let extension = match path.extension() {
                Some(extension) => extension,
                None => continue,
            };
            if extension == "jpg" || extension == "png" || extension == "jpeg"{
                image_files.push(path.to_path_buf());
            }
        }
    }

    if image_files.is_empty() {
        println!("No image files found in the directory.");
        return;
    }

    // Initialize the image viewer with the first image file
    let mut image_index = 0;
    let mut zoom_factor = 1.0;

    loop {
        // Display the current image
        let image = image::open(&image_files[image_index]).unwrap();
        display_image(image.clone(), zoom_factor);

        // Get the user input
        let input = get_user_input();

        match input.as_str() {
            "q" => {
                // Quit the image viewer
                break;
            }
            "n" => {
                // Display the next image
                if image_index < image_files.len() - 1 {
                    image_index += 1;
                    zoom_factor = 1.0;
                }else{
                    println!("This is the last image. There are no more images to display.");
                }
            }
            "p" => {
                // Display the previous image
                if image_index > 0 {
                    image_index -= 1;
                    zoom_factor = 1.0;
                }else{
                    println!("This is the first image. There are no previous.");
                }
            }
            "+" => {
                // Zoom in on the image
                zoom_factor += 0.2;
            }
            "-" => {
                // Zoom out from the image
                if zoom_factor > 0.2 {
                    zoom_factor -= 0.2;
                }else{
                    println!("The image is already zoomed out as far as it can go.");
                }
            }
            _ => {
                // Invalid input
                continue;
            }
        }
    }
}

fn get_user_input() -> String {
    println!("Press 'q' to quit, 'n' for the next image, 'p' for the previous image, '+' to zoom in, and '-' to zoom out.");
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


fn display_image(img: DynamicImage, zoom_factor: f64) {
    let (width, height) = img.dimensions();
    let mut resized_width = if width > 800 { 800 } else { width };
    let mut resized_height = (resized_width as f64 / width as f64 * height as f64) as u32;
    resized_width = (resized_width as f64 * zoom_factor) as u32;
    resized_height = (resized_height as f64 * zoom_factor) as u32;
    
    let resized_img = imageops::resize(&img, resized_width, resized_height, imageops::FilterType::Lanczos3);
    let mut buffer: Vec<u32> = vec![0; resized_width as usize * resized_height as usize];

    for (x, y, pixel) in resized_img.enumerate_pixels() {
        buffer[(y * resized_width + x) as usize] = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);
    }

    let mut window = Window::new(
        "Image Viewer",
        resized_width as usize,
        resized_height as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer,resized_width as usize,resized_height as usize).unwrap();
    }
}

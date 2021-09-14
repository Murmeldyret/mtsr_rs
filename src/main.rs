extern crate image;

use image::{DynamicImage, GenericImageView};
use imageproc::edges::canny;

use std::fs::{self, DirEntry};
use std::thread;

fn main() {
    let paths = detect_samples().unwrap();
    let images = load_samples(paths);
    let edge_images = detect_edges(images);
}

fn detect_samples() -> std::io::Result<Vec<DirEntry>> {
    let mut dir: Vec<DirEntry> = Vec::new();

    println!("Detecting samples...");

    for entry in fs::read_dir("./samples")? {
        dir.push(entry?);
    }
    Ok(dir)
}

fn load_samples(paths: Vec<DirEntry>) -> Vec<DynamicImage> {
    let mut img = Vec::new();

    println!("Loading samples to memory...");

    for i in 0..paths.len() {
        img.push(image::open(paths[i].path()).unwrap())
    }

    println!("Samples has been loaded.");
    println!("Current image dimension is: {:?}", img[0].dimensions());

    img
}

fn detect_edges(images: Vec<DynamicImage>) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    println!("Processing edges...");
    
    let mut edges;
    for i in 0..images.len() {
        
        thread::spawn(move || {
            canny(&images[i].to_luma8(), 30.0, 150.0);
        });    
    }
    

    edges
}

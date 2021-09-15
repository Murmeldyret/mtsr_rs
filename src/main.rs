extern crate image;

use image::{DynamicImage, GenericImageView};
use imageproc::edges::canny;

use futures::join;

use std::fs::{self, DirEntry};

fn main() {
    let paths = detect_samples().unwrap();
    let images = load_samples(paths);
    let _edge_images = detect_edges(images);
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
        img.push(image::open(paths[i].path()).unwrap());
        println!("{}/{} completed...", &i + 1, &paths.len());
    }

    println!("Samples has been loaded.");
    println!("Current image dimension is: {:?}", img[0].dimensions());

    img
}

async fn detect_edges(images: Vec<DynamicImage>) {
    println!("Processing edges...");
    let mut children = Vec::new();
    let mut edges = Vec::new();

    for i in 0..images.len() {
        children.push(thread_edges(images[i].to_luma8()));
        println!("Working on {}/{} images.", &i + 1, images.len());
    }

    for i in 0..images.len() {
        edges.push(join!(children[i]));
    }

    
}

async fn thread_edges(aimage: image::ImageBuffer<image::Luma<u8>, Vec<u8>>) {
    canny(&aimage, 30.0, 150.0);
}

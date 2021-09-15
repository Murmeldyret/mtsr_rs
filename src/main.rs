extern crate image;

use image::{DynamicImage, GenericImageView};
use imageproc::edges::canny;

use futures::executor::block_on;
use futures::{channel::mpsc, join};

use std::fs::{self, DirEntry};
use std::thread;

fn main() {
    let paths = detect_samples().unwrap();                                  //Returns the path of the samples.
    let images = load_samples(paths);                                    //Returns loaded images
    let edge_images = detect_edges(images);     //Returns edge detected images
    block_on(edge_images);
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
    let mut img = Vec::with_capacity(paths.len());

    println!("Loading samples to memory...");

    for i in 0..paths.len() {
        img.push(image::open(paths[i].path()).unwrap());
        println!("{}/{} completed...", &i + 1, &paths.len());
    }

    println!("Samples has been loaded.");
    println!("Current image dimension is: {:?}", img[0].dimensions());

    img
}

async fn detect_edges(
    images: Vec<DynamicImage>,
) -> (image::ImageBuffer<image::Luma<u8>, Vec<u8>>, image::ImageBuffer<image::Luma<u8>, Vec<u8>>, image::ImageBuffer<image::Luma<u8>, Vec<u8>>) {
    let edges1 = thread_edges(images[0].to_luma8());
    let edges2 = thread_edges(images[0].to_luma8());
    let edges3 = thread_edges(images[0].to_luma8());

    join!(edges1, edges2, edges3)

    /*
    println!("Processing edges...");
    let mut children = Vec::with_capacity(images.len());
    let mut edges = Vec::with_capacity(images.len());

    for i in 0..images.len() {
        children[i] = async {
            canny(&aimage, 30.0, 150.0);
            pool.spawn_
        }
        println!("Working on {}/{} images.", &i + 1, images.len());
    }

    edges = join!(children[0]);
    */
}

async fn thread_edges(
    aimage: image::ImageBuffer<image::Luma<u8>, Vec<u8>>,
) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    canny(&aimage, 30.0, 150.0)
}

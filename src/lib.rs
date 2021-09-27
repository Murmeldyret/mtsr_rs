mod image_processing;
mod samples;

use futures::executor::block_on;
use image::DynamicImage;

pub use crate::image_processing::process_images;
pub use crate::samples::loading;

pub fn load_images() -> Vec<image::DynamicImage> {
    let paths = loading::detect_samples().unwrap();
    loading::load_samples(paths)
}

pub fn process_images(images: Vec<DynamicImage>) {
    let edge_images = process_images::detect_edges(images);
    block_on(edge_images);
}

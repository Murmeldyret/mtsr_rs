use mtsr_rs::{load_images, process_images};

fn main() {
    let images = load_images(); //Returns loaded images
    let edge_images = process_images(images); //Returns edge detected images
}

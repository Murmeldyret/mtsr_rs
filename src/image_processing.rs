pub mod process_images {
    use std::{sync::Arc, thread};

    use image::DynamicImage;
    use imageproc::edges::canny;

    pub(crate) async fn detect_edges(images: Vec<DynamicImage>) {
        let mut handles = vec![];

        for i in 0..images.len() {
            let aimages = Arc::new(images[i].to_luma8());
            let handle = thread::spawn(move || canny(&aimages, 30.0, 150.0));
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

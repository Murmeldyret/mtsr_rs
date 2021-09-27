pub mod loading {
    use std::fs::{self, DirEntry};
    use image::{DynamicImage, GenericImageView};

    pub(crate) fn detect_samples() -> std::io::Result<Vec<DirEntry>> {
        let mut dir: Vec<DirEntry> = Vec::new();
    
        println!("Detecting samples...");
    
        for entry in fs::read_dir("./samples")? {
            dir.push(entry?);
        }
        Ok(dir)
    }
    
    pub(crate)fn load_samples(paths: Vec<DirEntry>) -> Vec<DynamicImage> {
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
}

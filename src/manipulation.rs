use std::collections::HashMap;

use image::imageops::FilterType;
use image::*;
use worker::*;

#[derive(Debug, Copy, Clone)]
struct ImageSize {
    width: u32,
    height: u32,
}

impl ImageSize {
    pub fn new(width: u32, height: u32) -> Self {
        ImageSize {
            width: width,
            height: height,
        }
    }
}

pub struct ManipulationParams {
    size: Option<ImageSize>,
    pub format: String,
}

impl ManipulationParams {
    fn new() -> Self {
        Self::with_data()
    }
    fn with_data() -> Self {
        Self {
            size: None,
            format: "png".to_string(),
        }
    }
    pub fn from_hash_map(params: &HashMap<String, String>) -> Self {
        let mut result = ManipulationParams::new();
        let width = match params.get("w") {
            Some(v) => v.parse::<u32>().unwrap_or(0),
            None => 0,
        };
        let height = match params.get("h") {
            Some(v) => v.parse::<u32>().unwrap_or(0),
            None => 0,
        };
        if width > 0 && height > 0 {
            result.size = Some(ImageSize::new(width, height))
        }
        result.format = params
            .get("fmt")
            .unwrap_or(&String::from("png"))
            .to_string();
        result
    }

    pub fn apply(&self, bytes: &Vec<u8>) -> Result<Vec<u8>> {
        let img = load_from_memory(&bytes).unwrap();
        let size = self.size.ok_or("no size")?;
        let modified_image = img.resize_exact(size.width, size.height, FilterType::Gaussian);
        let mut dst: Vec<u8> = Vec::new();
        let image_format: ImageOutputFormat = match self.format.as_ref() {
            "png" => ImageOutputFormat::Png,
            _ => ImageOutputFormat::Jpeg(80),
        };
        modified_image.write_to(&mut dst, image_format).unwrap();
        Ok(dst)
    }
}

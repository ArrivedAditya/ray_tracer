use std::{
    env, fs,
    path::{Path, PathBuf},
};

use zune_jpeg::{JpegDecoder, zune_core::bytestream::ZCursor};

struct Image {
    width: u32,
    height: u32,
    pub pixels: Vec<u8>,
}

impl Image {
    pub const EMPTY: Image = Image {
        width: 0,
        height: 0,
        pixels: Vec::new(),
    };

    pub fn new(filename: &str) -> Self {
        if let Ok(dir) = env::var("RTW_IMAGES") {
            let path = Path::new(&dir).join(filename);
            if let Some(img) = Self::load_file(&path) {
                return img;
            };
        }

        let mut path = PathBuf::from(filename);
        if let Some(img) = Self::load_file(&path) {
            return img;
        };

        path = Path::new("images").join(filename);
        for _ in 0..7 {
            if let Some(img) = Self::load_file(&path) {
                return img;
            };
            path = PathBuf::from("..").join(&path);
        }

        eprintln!("Error: Couldn't load image file {}", filename);
        Self::EMPTY
    }

    fn load_file(path: &Path) -> Option<Image> {
        let file_bytes = fs::read(path).unwrap();
        let mut decoder = JpegDecoder::new(ZCursor::new(&file_bytes));

        let pixels = decoder.decode().unwrap();
        let info = decoder.info()?;

        Some(Self {
            width: info.width as u32,
            height: info.height as u32,
            pixels,
        })
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> [u8; 3] {
        static MAGENTA: [u8; 3] = [255, 0, 255];
        if self.pixels.is_empty() {
            return MAGENTA;
        };

        let clamped_x = x.clamp(0, self.width.saturating_sub(x));
        let clamped_y = y.clamp(0, self.width.saturating_sub(y));

        let index = ((clamped_y * self.width + clamped_x) * 3) as usize;

        [
            self.pixels[index],
            self.pixels[index + 1],
            self.pixels[index + 2],
        ]
    }
}

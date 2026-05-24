use std::{
    env, fs,
    path::{Path, PathBuf},
};

use zune_jpeg::{JpegDecoder, zune_core::bytestream::ZCursor};

pub struct Image {
    pub width: u32,
    pub height: u32,
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
        let file_bytes = fs::read(path).ok()?;
        let mut decoder = JpegDecoder::new(ZCursor::new(&file_bytes));

        let pixels = decoder.decode().ok()?;
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

        let clamped_x = x.clamp(0, self.width.saturating_sub(1));
        let clamped_y = y.clamp(0, self.height.saturating_sub(1));

        let index = ((clamped_y * self.width + clamped_x) * 3) as usize;

        [
            self.pixels[index],
            self.pixels[index + 1],
            self.pixels[index + 2],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A valid, tiny 1x1 black pixel JPEG encoded as a raw byte array literal
    const TINY_JPEG: &[u8] = &[
        0xff, 0xd8, 0xff, 0xdb, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07, 0x06, 0x05, 0x08, 0x07,
        0x07, 0x07, 0x09, 0x09, 0x08, 0x0a, 0x0c, 0x14, 0x0d, 0x0c, 0x0b, 0x0b, 0x0c, 0x19, 0x12,
        0x13, 0x0f, 0x14, 0x1d, 0x1a, 0x1f, 0x1e, 0x1d, 0x1a, 0x1c, 0x1c, 0x20, 0x24, 0x2e, 0x27,
        0x20, 0x22, 0x2c, 0x23, 0x1c, 0x1c, 0x28, 0x37, 0x29, 0x2c, 0x30, 0x31, 0x34, 0x34, 0x34,
        0x1f, 0x27, 0x39, 0x3d, 0x38, 0x32, 0x3c, 0x2e, 0x33, 0x34, 0x32, 0xff, 0xc0, 0x00, 0x0b,
        0x08, 0x00, 0x01, 0x00, 0x01, 0x01, 0x01, 0x11, 0x00, 0xff, 0xc4, 0x00, 0x14, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xff, 0xc4, 0x00, 0x14, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xda, 0x00, 0x08, 0x01, 0x01, 0x00,
        0x00, 0x3f, 0x00, 0xbf, 0x00, 0xff, 0xd9,
    ];

    #[test]
    fn test_empty_image_returns_magenta() {
        let img = Image::EMPTY;
        assert_eq!(img.width, 0);
        assert_eq!(img.height, 0);

        // Querying any pixel on an uninitialized/failed image must safely return Magenta
        let pixel = img.pixel_data(0, 0);
        assert_eq!(pixel, [255, 0, 255]);
    }

    #[test]
    fn test_decoder_with_valid_data() {
        // We bypass the file hunter logic here to test if the zune-jpeg integration works perfectly
        let mut decoder = JpegDecoder::new(ZCursor::new(TINY_JPEG));
        let pixels = decoder.decode().expect("Failed to decode tiny test JPEG");
        let info = decoder.info().unwrap();

        let img = Image {
            width: info.width as u32,
            height: info.height as u32,
            pixels,
        };

        assert_eq!(img.width, 1);
        assert_eq!(img.height, 1);
        assert_eq!(img.pixels.len(), 3); // 1 pixel * 3 channels (RGB)
    }

    #[test]
    fn test_pixel_clamping_safety() {
        // Build a fake 2x2 image manually to verify our math formula doesn't crash on edges
        // Row 0: Red, Green | Row 1: Blue, White
        let fake_pixels = vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255];

        let img = Image {
            width: 2,
            height: 2,
            pixels: fake_pixels,
        };

        // Test normal inside-bounds reads
        assert_eq!(img.pixel_data(0, 0), [255, 0, 0]); // Top-left (Red)
        assert_eq!(img.pixel_data(1, 1), [255, 255, 255]); // Bottom-right (White)

        // Crucial: Test out-of-bounds queries. They should safely clamp to the edge pixel instead of panicking.
        assert_eq!(img.pixel_data(5, 0), [0, 255, 0]); // Far right -> Clamps to (1,0) (Green)
        assert_eq!(img.pixel_data(0, 5), [0, 0, 255]); // Far bottom -> Clamps to (0,1) (Blue)
        assert_eq!(img.pixel_data(10, 10), [255, 255, 255]); // Extreme out of bounds -> Clamps to (1,1)
    }

    #[test]
    fn test_missing_file_fails_gracefully() {
        // Attempting to look up a non-existent file path should return an empty struct, not panic
        let img = Image::new("this_file_definitely_does_not_exist.jpg");
        assert_eq!(img.width, 0);
        assert_eq!(img.height, 0);
        assert!(img.pixels.is_empty());
    }
}

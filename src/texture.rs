use std::sync::Arc;

use crate::{
    color::Color,
    image::{self, Image},
    interval::Interval,
    vec3::Point3,
};

pub type TexturePtr = Arc<dyn Texture + Send + Sync>;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            albedo: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerPattern {
    inv_scale: f32,
    even: TexturePtr,
    odd: TexturePtr,
}

impl CheckerPattern {
    pub fn new(scale: f32, even: TexturePtr, odd: TexturePtr) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn form_colors(scale: f32, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerPattern {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let x = (self.inv_scale * p.x).floor() as i32;
        let y = (self.inv_scale * p.y).floor() as i32;
        let z = (self.inv_scale * p.z).floor() as i32;

        let is_even = if (x + y + z) % 2 == 0 { true } else { false };

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: Image,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: Image::new(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        // If no texture data here then show cyan.
        if self.image.height == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width as f32) as u32;
        let j = (v * self.image.height as f32) as u32;

        let color_scale = 1.0 / 255.0;
        let pixel = self.image.pixel_data(i, j);

        color_scale * Color::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32)
    }
}

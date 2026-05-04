use std::io::{self, Write};

#[derive(Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) -> io::Result<()> {
    let rbyte = (255.999 * pixel_color.r) as u8;
    let gbyte = (255.999 * pixel_color.g) as u8;
    let bbyte = (255.999 * pixel_color.b) as u8;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;

    Ok(())
}

use std::ops::{Add, Mul};

impl Add for Color {
    type Output = Color;
    fn add(self, v: Color) -> Color {
        Color::new(self.r + v.r, self.g + v.g, self.b + v.b)
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, t: f32) -> Color {
        Color::new(self.r * t, self.g * t, self.b * t)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, c: Color) -> Color {
        c * self
    }
}

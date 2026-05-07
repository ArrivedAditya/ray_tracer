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

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    0.0
}

pub fn write_color<W: Write>(
    out: &mut W,
    intensity: &Interval,
    pixel_color: Color,
) -> io::Result<()> {
    // gamma needed for smooth lighting accuracy
    let r = linear_to_gamma(pixel_color.r);
    let g = linear_to_gamma(pixel_color.g);
    let b = linear_to_gamma(pixel_color.b);

    let rbyte = (256.0 * intensity.clamp(r)) as u8;
    let gbyte = (256.0 * intensity.clamp(g)) as u8;
    let bbyte = (256.0 * intensity.clamp(b)) as u8;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;

    Ok(())
}

use std::ops::{Add, AddAssign, Mul};

use crate::interval::Interval;

impl Add for Color {
    type Output = Color;
    fn add(self, v: Color) -> Color {
        Color::new(self.r + v.r, self.g + v.g, self.b + v.b)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
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

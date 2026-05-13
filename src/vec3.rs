// Vec3 & Point3 form scratch.

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        self.x.abs() < s && self.y.abs() < s && self.z < s
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    // direction of vector
    pub fn unit_vector(&self) -> Vec3 {
        Vec3::new(
            self.x / self.length(),
            self.y / self.length(),
            self.z / self.length(),
        )
    }
}

pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::rng();
    loop {
        let p = Vec3::new(
            rng.random_range(-1.0..=1.0),
            rng.random_range(-1.0..=1.0),
            rng.random_range(-1.0..=1.0),
        );

        let lensq = p.length_squared();

        if lensq > 1e-24 && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (n * (2.0 * v.dot(&n)))
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(n.dot(&(-1.0 * uv)), 1.0);
    let r_out_prep = etai_over_etat * (uv + (cos_theta * n));
    let out_parallel = -(f32::abs(1.0 - r_out_prep.length_squared()).sqrt() * n);
    r_out_prep + out_parallel
}

pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let p = Vec3::new(
            rng.random_range(-1.0..=1.0),
            rng.random_range(-1.0..=1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

// for coordiantes
pub type Point3 = Vec3;

// Defining the operation of Vec3
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::{RngExt, rngs::ThreadRng};

// vec1 + vec2
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

// vec1 += vec2
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

// vec1 - vec2
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// vec1 * vec2
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

// vec * t
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f32) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

// t * vec
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// vec *= t
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

// vec / t
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f32) -> Vec3 {
        self * (1.0 / t)
    }
}

// vec /= t
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        self.x *= 1.0 / t;
        self.y *= 1.0 / t;
        self.z *= 1.0 / t;
    }
}

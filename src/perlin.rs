use fastrand::Rng;

use crate::vec3::{Point3, Vec3, random_unit_vector};

const POINT_COUNT: usize = 256;

pub struct PerlinNoise {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl PerlinNoise {
    pub fn new(rng: &mut Rng) -> Self {
        let mut randvec: [Vec3; POINT_COUNT] = [Vec3::default(); POINT_COUNT];

        for i in 0..POINT_COUNT {
            randvec[i] = random_unit_vector(rng, -1.0, 1.0);
        }
        let mut perm_x = [0; POINT_COUNT];
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];

        Self::generate_perm(&mut perm_x, rng);
        Self::generate_perm(&mut perm_y, rng);
        Self::generate_perm(&mut perm_z, rng);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        let i = ((p.x.floor() as i32) & 255) as usize;
        let j = ((p.y.floor() as i32) & 255) as usize;
        let k = ((p.z.floor() as i32) & 255) as usize;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dy in 0..2 {
                for dk in 0..2 {
                    c[di][dy][dk] = self.randvec[self.perm_x[(i + di) & 255] as usize
                        ^ self.perm_y[(j + dy) & 255] as usize
                        ^ self.perm_z[(k + dk) & 255] as usize];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    // turbuance fn
    pub fn turb(&self, p: &Point3, depth: i32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn generate_perm(p: &mut [i32; POINT_COUNT], rng: &mut Rng) {
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }

        Self::permute(p, rng);
    }

    fn permute(p: &mut [i32; POINT_COUNT], rng: &mut Rng) {
        for i in (0..(POINT_COUNT)).rev() {
            let target = rng.usize(0..=i);
            p.swap(i, target);
        }
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                        * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                        * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }
}

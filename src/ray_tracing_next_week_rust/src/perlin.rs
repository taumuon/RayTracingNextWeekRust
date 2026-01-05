use crate::vec3::{Vec3, dot};
use crate::rtweekend::*;
use rand::seq::SliceRandom;
const POINT_COUNT: usize = 256;

pub struct Perlin {
    perm_x : [i32;POINT_COUNT],
    perm_y : [i32;POINT_COUNT],
    perm_z : [i32;POINT_COUNT],
    // randFloat: [f64;POINT_COUNT],
    rand_vec: [Vec3;POINT_COUNT]
}

impl Perlin {
    pub fn new() -> Self {
        let mut perm_x = [0 as i32; POINT_COUNT];
        let mut perm_y = [0 as i32; POINT_COUNT];
        let mut perm_z = [0 as i32; POINT_COUNT];
        // let mut randFloat= [0.0; POINT_COUNT];
        let mut rand_vec= [Vec3::new(0.0, 0.0, 0.0); POINT_COUNT];

        for n in 0..POINT_COUNT {
            rand_vec[n] = random_vec3_min_max(-1.0, 1.0).normalized();
            perm_x[n] = n as i32;
            perm_y[n] = n as i32;
            perm_z[n] = n as i32;
        }

        let mut rng = rand::rng();
        perm_x[..].shuffle(&mut rng);
        perm_y[..].shuffle(&mut rng);
        perm_z[..].shuffle(&mut rng);

        return Self {
            perm_x, perm_y, perm_z, /*randFloat,*/ rand_vec
        };
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x as i64;
        let j = p.y as i64;
        let k = p.z as i64;

        let mut c = [Vec3::new(0.0, 0.0, 0.0); 8];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = self.perm_x[((i + di) & 255) as usize]
                     ^ self.perm_y[((j + dj) & 255) as usize]
                     ^ self.perm_z[((k + dk) & 255) as usize];
                    c[(di * 4 + dj * 2 + dk) as usize] = self.rand_vec[idx as usize];
                }
            }
        }

        return Self::perlin_interp(&c[..], u, v, w);
    }

    fn perlin_interp(c: &[Vec3], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            let i_f = i as f64;
            for j in 0..2 {
                let j_f = j as f64;
                for k in 0..2 {
                    let k_f = k as f64;
                    let weight_v = Vec3::new(u - i_f, v - j_f, w - k_f);
                    accum += (i_f * uu + (1.0 - i_f)*(1.0 - uu))
                    * (j_f * vv + (1.0 - j_f)*(1.0 - vv))
                    * (k_f * ww + (1.0 - k_f)*(1.0 - ww))
                    * dot(&c[(i * 4 + j * 2 + k) as usize], &weight_v);
                }
            }
        }

        return accum;
    }

    pub fn turb(&self, p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p;
        }

        return accum.abs();
    }
}
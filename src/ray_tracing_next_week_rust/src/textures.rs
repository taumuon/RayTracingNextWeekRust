use std::sync::Arc;

use crate::texture::Texture;
use crate::vec3::Vec3;
use crate::perlin::Perlin;

pub struct SolidColorTexture {
    albedo: Vec3
}

impl SolidColorTexture {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColorTexture {
    fn color_value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        return self.albedo;
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        let inv_scale = 1.0 / scale;
        return Self {inv_scale, even, odd};
    }
}

impl Texture for CheckerTexture {
    fn color_value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let u_integer = (self.inv_scale * u).floor() as u64;
        let v_integer = (self.inv_scale * v).floor() as u64;

        let is_even = (u_integer + v_integer) % 2 == 0;
        if is_even {
            return self.even.color_value(u, v, p);
        }
        return self.odd.color_value(u, v, p);
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        let noise = Perlin::new();
        return Self {
            noise, scale
        }
    }
}

impl Texture for NoiseTexture {
    fn color_value(&self, _u: f64, _v: f64, p: Vec3) -> Vec3 {
        return 
            (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
            * Vec3::new(0.5, 0.5, 0.5);
    }
}
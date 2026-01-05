use load_image::export::rgb::Rgba;
use std::str;
use crate::texture::Texture;
use crate::vec3::Vec3;
use crate::interval::Interval;

pub struct ImageTexture {
    image: Vec<Rgba<u8>>,
    width: usize,
    height: usize
}

impl ImageTexture {
    pub fn new(path: &str) -> Self {
        let img = load_image::load_path(path).unwrap();

        let width = img.width;
        let height = img.height;
        let (rgba, _meta) = img.into_rgba();
        let (buf, _w, _h) = rgba.into_contiguous_buf();

        return Self {image: buf, width: width, height: height};
    }
}

impl Texture for ImageTexture {
    fn color_value(&self, u: f64, v: f64, _p: Vec3) -> Vec3 {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.height <= 0 { return Vec3::new(0.0,1.0,1.0); }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);  // Flip V to image coordinates

        let i = (u * self.width as f64) as usize;
        let j = (v * self.height as f64) as usize;
        let pixel = self.image[self.width * j + i];

        let color_scale = 1.0 / 255.0;
        return Vec3::new(color_scale * pixel.r as f64, color_scale * pixel.g as f64, color_scale * pixel.b as f64);
    }
}
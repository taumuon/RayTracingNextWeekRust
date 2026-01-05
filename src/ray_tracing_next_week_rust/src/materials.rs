use std::sync::Arc;
use crate::texture::Texture;

use crate::vec3;
use crate::vec3::Vec3;
use crate::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::hitrecord::HitRecord;
use crate::rtweekend::{self, random_unit_vector};

pub struct Lambertian {
    texture: Arc<dyn Texture>
}

pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

pub struct DiffuseLight {
    texture: Arc<dyn Texture>
}

pub struct Isotropic {
    texture: Arc<dyn Texture>
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self {
            texture
        }
    }
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo, fuzz
        }
    }
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index
        }
    }
}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self {
            texture
        }
    }
}

impl Isotropic {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self {
            texture
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new2(hit_record.p, scatter_direction, r_in.tm);
        let attenuation = self.texture.color_value(hit_record.u, hit_record.v, hit_record.p);
        let scatter_result = ScatterResult::new(attenuation, scattered);
        return Some(scatter_result);
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let reflected = rtweekend::reflect(r_in.direction, hit_record.normal);
        let fuzzy_reflected = reflected.normalized() + (self.fuzz * rtweekend::random_unit_vector());

        if vec3::dot(&reflected, &hit_record.normal) > 0.0 {
            let scattered = Ray::new2(hit_record.p, fuzzy_reflected, r_in.tm);
            let scatter_result = ScatterResult::new(self.albedo, scattered);
            return Some(scatter_result);
        }

        return None;
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ri = if hit_record.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = r_in.direction.normalized();

        let cos_theta = f64::min(vec3::dot(&-unit_direction, &hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract
         || reflectance(cos_theta, ri ) > rtweekend::random_double() {
             rtweekend::reflect(unit_direction, hit_record.normal)
            }
         else {
             rtweekend::refract(unit_direction, hit_record.normal, ri)
            };

        let scattered = Ray::new2(hit_record.p, direction, r_in.tm);
        let scatter_result = ScatterResult::new(attenuation, scattered);
        return Some(scatter_result);
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _hit_record: &HitRecord) -> Option<ScatterResult> {
        return None;
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        return self.texture.color_value(u, v, p);
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let attenuation = self.texture.color_value(hit_record.u, hit_record.v, hit_record.p);
        let scattered = Ray::new2(hit_record.p, random_unit_vector(), r_in.tm);
        let scatter_result = ScatterResult::new(attenuation, scattered);
        return Some(scatter_result);
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0_squared = r0 * r0;
    return r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powf(5.0);
}

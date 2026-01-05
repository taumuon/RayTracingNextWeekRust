use crate::vec3::{self, Vec3};

use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_double() -> f64 {
    // Returns a random double in [0, 1).
    return rand::random();
}

pub fn random_double_min_max(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    return min + (max - min) * random_double();
}

pub fn random_vec3() -> Vec3 {
    return Vec3::new(random_double(), random_double(), random_double());
}

pub fn random_vec3_min_max(min: f64, max: f64) -> Vec3 {
    return Vec3::new(random_double_min_max(min, max), random_double_min_max(min, max), random_double_min_max(min, max));
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = random_vec3_min_max(-1.0, 1.0);
        let len_sqr = p.length_squared();
        if 1e-6 < len_sqr && len_sqr <= 1.0 {
            return p / len_sqr.sqrt();
        }
    }
}

pub fn random_in_unit_disc() -> Vec3 {
    loop {
        let p = Vec3::new(random_double_min_max(-1.0, 1.0), random_double_min_max(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * vec3::dot(&v,&n)*n;
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(vec3::dot(&-uv, &n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(f64::abs(1.0 - r_out_perp.length_squared()).sqrt()) * n;
    return r_out_perp + r_out_parallel;
}
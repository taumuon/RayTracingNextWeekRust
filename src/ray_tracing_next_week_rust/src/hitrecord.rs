use std::sync::Arc;

use crate::vec3;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

// #[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
    pub material: Arc<dyn Material + Sync>
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Vec3, outward_normal: Vec3, t: f64, u: f64, v: f64, material: Arc<dyn Material>) -> Self {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        let front_face = vec3::dot(&ray.direction, &outward_normal) < 0.0;
        let normal = if front_face { outward_normal} else {-outward_normal};
        Self {
            p, normal, t, front_face, u, v, material
        }
    }

     pub fn new2(p: Vec3, t: f64, u: f64, v: f64, material: Arc<dyn Material>) -> Self {
        let front_face = true;
        let normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        Self {
            p, normal, t, front_face, u, v, material
        }
    }
}
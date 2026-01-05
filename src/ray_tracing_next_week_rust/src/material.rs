use crate::ray::Ray;
use crate::hitrecord::HitRecord;
use crate::vec3::Vec3;

pub struct ScatterResult {
    pub attenuation: Vec3,
    pub scattered: Ray
}

impl ScatterResult {
    pub fn new(attenuation: Vec3, scattered: Ray) -> Self {
        Self {
            attenuation, scattered
        }
    }    
}

pub trait Material : Send + Sync {
    fn scatter(&self, _r_in: &Ray, _hit_record: &HitRecord) -> Option<ScatterResult>;

    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
}
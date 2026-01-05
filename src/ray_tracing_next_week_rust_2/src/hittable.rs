use crate::ray::Ray;
use crate::interval::Interval;
use crate::hitrecord::HitRecord;
use crate::aabb::AABB;

pub trait Hittable : Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;
}
use std::sync::Arc;
use crate::hittable::Hittable;
use crate::hitrecord::HitRecord;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::aabb::AABB;

pub struct Translate {
    offset: Vec3,
    bbox: AABB,
    hittable: Arc<dyn Hittable>
}

impl Translate {
    pub fn new(offset: Vec3, hittable: Arc<dyn Hittable>) -> Self {
        let bbox = hittable.bounding_box() + offset;
        return Self {
            offset, bbox, hittable
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Move the ray backwards by the offset
        let offset_r = Ray::new2(ray.origin - self.offset, ray.direction, ray.tm);

        // Determine whether an intersection exists along the offset ray (and if so, where)
        let hit_result = self.hittable.hit(&offset_r, ray_t);
        if let Some(hit) = hit_result {
            // Move the intersection point forwards by the offset
            let p = hit.p + self.offset;
            return Some(HitRecord::new(ray, p, hit.normal, hit.t, hit.u, hit.v, hit.material));
        }

        return None;
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}
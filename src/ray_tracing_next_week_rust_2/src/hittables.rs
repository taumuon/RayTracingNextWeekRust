use std::sync::Arc;

use crate::hittable::Hittable;
use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::aabb::AABB;

pub struct Hittables {
    objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB
}

impl Hittables {
    pub fn new(objects_slice: &[Arc<dyn Hittable>]) -> Self {
        let objects = objects_slice.to_vec();
        let mut bbox = AABB::empty();
        for obj in objects.iter() {
            bbox = AABB::from_aabb(&bbox, &obj.bounding_box());
        }

        Self {
            objects, bbox
        }
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = Option::None;

        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            let object_hit_result = object.hit(ray, &Interval::new(ray_t.min, closest_so_far));
            if let Some(object_hit) = object_hit_result {
                closest_so_far = object_hit.t;
                hit_record = Some(object_hit);
            }
        }

        return hit_record;
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}
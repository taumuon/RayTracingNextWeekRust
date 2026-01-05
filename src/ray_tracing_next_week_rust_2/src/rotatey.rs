use std::sync::Arc;
use crate::hittable::Hittable;
use crate::hitrecord::HitRecord;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::aabb::AABB;
use crate::rtweekend::degrees_to_radians;

pub struct RotateY {
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
    hittable: Arc<dyn Hittable>
}

impl RotateY {
    pub fn new(angle: f64, hittable: Arc<dyn Hittable>) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = hittable.bounding_box();

        let mut min_x = f64::INFINITY;
        //let mut min_y = f64::INFINITY;
        let mut min_z = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        //let mut max_y = f64::NEG_INFINITY;
        let mut max_z = f64::NEG_INFINITY;

        for i in 0..2 {
            for k in 0..2 {
                let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                let new_x = cos_theta * x + sin_theta * z;
                let new_z = -sin_theta * x + cos_theta * z;

                min_x = new_x.min(min_x);
                max_x = new_x.max(max_x);
                min_z = new_z.min(min_z);
                max_z = new_z.max(max_z);
            }
        }

        let min = Vec3::new(min_x, bbox.y.min, min_z);
        let max = Vec3::new(max_x, bbox.y.max, max_z);
        let bbox = AABB::from_points(&min, &max);

        return Self {
            sin_theta, cos_theta, bbox, hittable
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Translate the ray from world space to object space.

        let origin = Vec3::new(
            (self.cos_theta * ray.origin.x) - (self.sin_theta * ray.origin.z),
            ray.origin.y,
            (self.sin_theta * ray.origin.x) + (self.cos_theta * ray.origin.z)
        );

        let direction = Vec3::new(
            (self.cos_theta * ray.direction.x) - (self.sin_theta * ray.direction.z),
            ray.direction.y,
            (self.sin_theta * ray.direction.x) + (self.cos_theta * ray.direction.z)
        );

        let rotated_r = Ray::new2(origin, direction, ray.tm);

        // Determine whether an intersection exists in object space (and if so, where).
        let hit_result = self.hittable.hit(&rotated_r, ray_t);
        if let Some(hit) = hit_result {
            // Transform the intersection from object space back to world space.

            let p = Vec3::new(
                (self.cos_theta * hit.p.x) + (self.sin_theta * hit.p.z),
                hit.p.y,
                (-self.sin_theta * hit.p.x) + (self.cos_theta * hit.p.z)
            );

            let normal = Vec3::new(
                (self.cos_theta * hit.normal.x) + (self.sin_theta * hit.normal.z),
                hit.normal.y,
                (-self.sin_theta * hit.normal.x) + (self.cos_theta * hit.normal.z)
            );
            
            return Some(HitRecord::new(ray, p, normal, hit.t, hit.u, hit.v, hit.material));
        }

        return None;
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}
use std::sync::Arc;

use crate::vec3::{Vec3, dot, cross};
use crate::aabb::AABB;
use crate::material::Material;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::hitrecord::HitRecord;
use crate::interval::Interval;

pub struct Quad{
    q: Vec3,
    u: Vec3,
    v: Vec3,
    material: Arc<dyn Material>,
    bbox: AABB,
    d: f64,
    w: Vec3,
    normal: Vec3
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let n = cross(&u, &v);
        let normal = n.normalized();
        let d = dot(&normal, &q);
        let w = n / dot(&n, &n);

        // Compute the bounding box of all four vertices.
        let bbox_diagonal1 = AABB::from_points(&q, &(q + u + v));
        let bbox_diagonal2 = AABB::from_points(&(q + u), &(q + v));
        let bbox = AABB::from_aabb(&bbox_diagonal1, &bbox_diagonal2);

        Self {
            q, u, v, material, bbox, d, w, normal
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denom = dot(&self.normal, &ray.direction);

        if denom.abs() < 1e-8 {
            // No hit if the ray is parallel to the plane.
            return None;
        }

        let t = (self.d - dot(&self.normal, &ray.origin)) / denom;
        if !ray_t.contains(t) {
            // Return false if the hit point parameter t is outside the ray interval.
            return None;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);

        let planar_hit_pt_vec = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hit_pt_vec, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hit_pt_vec));

        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return None;
        }

        return Some(HitRecord::new(ray, intersection, self.normal, t, alpha, beta, Arc::clone(&self.material)));
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}

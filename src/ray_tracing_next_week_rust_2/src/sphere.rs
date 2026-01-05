use crate::vec3::Vec3;
use crate::vec3;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hitrecord::HitRecord;
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::AABB;
use std::f64;
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    material: i16,
    bbox: AABB
}

impl Sphere {
    // stationary
    pub fn new(center: Vec3, radius: f64, material: i16) -> Self {
        let max_radius = radius.max(0.0);
        let ray = Ray::new(center, Vec3::new(0.0, 0.0, 0.0));
        let rvec = Vec3::new(radius, radius, radius);
        let aabb = AABB::from_points(&(center - rvec), &(center + rvec));
        Self {
            center: ray, radius: max_radius, material, bbox: aabb
        }
    }

    // moving
    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f64, material: i16) -> Self {
        let max_radius = radius.max(0.0);
        let ray = Ray::new(center1, center2 - center1);
        let rvec = Vec3::new(radius, radius, radius);
        let bbox1 = AABB::from_points(&(ray.at(0.0) - rvec), &(ray.at(0.0) + rvec));
        let bbox2 = AABB::from_points(&(ray.at(1.0) - rvec), &(ray.at(1.0) + rvec));
        let aabb = AABB::from_aabb(&bbox1, &bbox2);
        Self {
            center: ray, radius: max_radius, material, bbox: aabb
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.tm);
        let oc = current_center - ray.origin;
        let a = ray.direction.length_squared();
        let h = vec3::dot(&ray.direction, &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(root);
        let outward_normal = (p - current_center) / self.radius;
        let (u, v) = get_sphere_uv(outward_normal);
        let rec = HitRecord::new(ray, p, outward_normal, t, u, v, self.material);

        return Some(rec);
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}

fn get_sphere_uv(p: Vec3) -> (f64, f64) {
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + std::f64::consts::PI;

    let u = phi / (2.0 * std::f64::consts::PI);
    let v = theta / std::f64::consts::PI;
    return (u, v);
}
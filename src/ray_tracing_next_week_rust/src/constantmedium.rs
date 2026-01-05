use std::sync::Arc;
use crate::hittable::Hittable;
use crate::texture::Texture;
use crate::material::Material;
use crate::materials::Isotropic;
use crate::ray::Ray;
use crate::hitrecord::HitRecord;
use crate::interval::Interval;
use crate::aabb::AABB;
use crate::rtweekend::random_double;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable + Sync>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, tex: Arc<dyn Texture>, density: f64) -> Self {
        let phase_function: Arc<dyn Material + Sync> = Arc::new(Isotropic::new(tex));
        let neg_inv_density = -1.0 / density;
        return Self {
            boundary, neg_inv_density, phase_function
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let hit_result = &self.boundary.hit(ray, &Interval::universe());
        if let Some(hit) = hit_result {
            let hit_result2 = &self.boundary.hit(ray, &Interval::new(hit.t + 0.0001, f64::INFINITY));
            if let Some(hit2) = hit_result2
            {
                let mut hit_t1 = hit.t.max(ray_t.min);
                let hit_t2 = hit2.t.min(ray_t.max);
                if hit_t1 >= hit_t2 {
                    return None;
                }

                if hit_t1 < 0.0 {
                    hit_t1 = 0.0;
                }

                let ray_length = ray.direction.length();
                let distance_inside_boundary = (hit_t2 - hit_t1) * ray_length;
                let rnd = random_double();
                let rnd_ln = rnd.ln();
                let hit_distance = self.neg_inv_density * rnd_ln;

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = hit_t1 + hit_distance / ray_length;
                let p = ray.at(t);

                // TODO: HitRecord should be an enum
                return Some(HitRecord::new2(
                 p,
                t,
                0.0, 
                0.0,
                Arc::clone(&self.phase_function)));
            }
        }

        return None;
    }

    fn bounding_box(&self) -> AABB {
        return self.boundary.bounding_box();
    }
}
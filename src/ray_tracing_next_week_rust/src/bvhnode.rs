use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::hitrecord::HitRecord;
use crate::aabb::AABB;
use std::sync::Arc;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB
}

impl BvhNode {
    pub fn new(objects_slice: &[Arc<dyn Hittable>]) -> Self {
        let mut objects = objects_slice.to_vec();
        if objects.len() == 1 {
            return Self {
                left: Arc::clone(&objects[0]), right: Arc::clone(&objects[0]), bbox: objects[0].bounding_box()
            }
        }

        if objects.len() == 2 {
            let aabb = AABB::from_aabb(&objects[0].bounding_box(), &objects[1].bounding_box());
            return Self {
                left: Arc::clone(&objects[0]), right: Arc::clone(&objects[1]), bbox: aabb
            }
        }

        // Build the bounding box of the span of source objects.
        let mut bbox = AABB::empty();
        for obj in objects.iter() {
            let obj_bbox = &obj.bounding_box();
            bbox = AABB::from_aabb(&bbox, obj_bbox);
        }

        let axis = bbox.longest_axis();

        objects.sort_by(|a,b|
            {
                let aaxis = a.bounding_box().axis_interval(axis);
                let baxis = b.bounding_box().axis_interval(axis);
                aaxis.min.partial_cmp(&baxis.min).unwrap()
            });

        let mid = objects.len() / 2;

        let left_objects = &objects[..mid];
        let right_objects = &objects[mid..];

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if left_objects.len() == 1 {
            left = Arc::clone(&left_objects[0]);
        }
        else {
            left = Arc::new(BvhNode::new(left_objects));    
        }
        if right_objects.len() == 1 {
            right = Arc::clone(&right_objects[0]);
        }
        else {
            right = Arc::new(BvhNode::new(right_objects));
        }

        return Self {
            left, right, bbox
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, ray_t) {
            return None;
        }

        let left_hit = self.left.hit(ray, ray_t);

        let right_t_max = if let Some(ref left_hit_r) = left_hit { left_hit_r.t } else { ray_t.max };
        let right_t = Interval::new(ray_t.min, right_t_max);

        let right_hit = self.right.hit(ray, &right_t);

        if let Some (ref _right_hit_result) = right_hit {
            return right_hit;
        }
        if let Some (ref _left_hit_result) = left_hit {
            return left_hit;
        }

        return None;
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}
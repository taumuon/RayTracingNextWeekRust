use std::ops::Add;
use crate::interval::Interval;
use crate::vec3::Vec3;
use crate::ray::Ray;

use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval
}

impl AABB {
    pub fn empty() -> Self {
        Self {
            x: Interval::empty(), y: Interval::empty(), z: Interval::empty()
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self {
            x, y, z
        }
    }

    pub fn from_aabb(box0: &AABB, box1: &AABB) -> Self {
        let x = Interval::from_intervals_padded(&box0.x, &box1.x);
        let y = Interval::from_intervals_padded(&box0.y, &box1.y);
        let z = Interval::from_intervals_padded(&box0.z, &box1.z);
        Self {
            x, y, z
        }
    }

    pub fn from_points(a: &Vec3, b: &Vec3) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.

        let x = if a.x <= b.x { Interval::new_padded(a.x, b.x) } else { Interval::new_padded(b.x, a.x) };
        let y = if a.y <= b.y { Interval::new_padded(a.y, b.y) } else { Interval::new_padded(b.y, a.y) };
        let z = if a.z <= b.z { Interval::new_padded(a.z, b.z) } else { Interval::new_padded(b.z, a.z) };

        Self {
            x, y, z
        }
    }

    pub fn axis_interval(&self, n: u16) -> Interval {
        if n == 1 { return self.y }
        if n == 2 { return self.z }
        return self.x;
    }

    pub fn longest_axis(&self) -> u16 {
        // Returns the index of the longest axis of the bounding box.

        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();
        if x_size > y_size {
            if x_size > z_size { return 0; } else { if z_size > y_size {return 2;} else {return 1;} }
        }
        else {
            if y_size > z_size { return 1; } else { if z_size > x_size {return 2;} else {return 0;} }
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let ray_origin = ray.origin;
        let ray_dir = ray.direction;

        let mut ray_t_min = ray_t.min;
        let mut ray_t_max = ray_t.max;
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir.component(axis);

            let t0 = (ax.min - ray_origin.component(axis)) * adinv;
            let t1 = (ax.max - ray_origin.component(axis)) * adinv;

            if t0 < t1 {
                if t0 > ray_t_min {
                    ray_t_min = t0;
                }
                if t1 < ray_t_max {
                    ray_t_max = t1;
                }
            }
            else {
                if t1 > ray_t_min {
                    ray_t_min = t1;
                }
                if t0 < ray_t_max {
                    ray_t_max = t0;
                }
            }

            if ray_t_max <= ray_t_min {
                return false;
            }
        }

        return true;
    }
}

impl Add<Vec3> for AABB {
    type Output = AABB;

    fn add(self, rhs: Vec3) -> AABB {
        AABB { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Add<AABB> for Vec3 {
    type Output = AABB;

    fn add(self, rhs: AABB) -> AABB {
        AABB { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl fmt::Display for AABB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{:.3} y:{:.3} z:{:.3}", self.x, self.y, self.z)
    }
}
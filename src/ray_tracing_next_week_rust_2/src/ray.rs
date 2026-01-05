use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub tm: f64
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin, direction, tm:0.0
        }
    }
    
    pub fn new2(origin: Vec3, direction: Vec3, tm: f64) -> Self {
        Self {
            origin, direction, tm
        }
    }

    pub fn at(&self, t: f64) -> Vec3
    {
        return self.origin + t * self.direction;
    }
}
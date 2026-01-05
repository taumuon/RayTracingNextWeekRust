use core::f64;
use std::fmt;
use std::ops::Add;

const DELTA: f64 = 0.0001;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY, max: f64::NEG_INFINITY
        }
    }

    pub fn universe() -> Self {
        Self {
            min: f64::NEG_INFINITY, max: f64::INFINITY
        }
    }

    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min, max
        }
    }

    pub fn new_padded(min: f64, max: f64) -> Self {
        let size = (max - min).abs();
        if size < DELTA {
            return Self {
                min: min - DELTA / 2.0, max: max + DELTA / 2.0
            };
        };
        Self {
            min, max
        }
    }

    pub fn from_intervals(a: &Interval, b: &Interval) -> Self {
        // Create the interval tightly enclosing the two input intervals.
        let min = a.min.min(b.min);
        let max = a.max.max(b.max);
        Self {
            min, max
        }
    }

    pub fn from_intervals_padded(a: &Interval, b: &Interval) -> Self {
        // Create the interval tightly enclosing the two input intervals.
        let min = a.min.min(b.min);
        let max = a.max.max(b.max);
        let size = (max - min).abs();
        if size < DELTA {
            return Self {
                min: min - DELTA / 2.0, max: max + DELTA / 2.0
            };
        };
        Self {
            min, max
        }
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {return self.min};
        if x > self.max {return self.max};
        return x;
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding, max: self.max + padding
        }
    }
}

impl Add<f64> for Interval {
    type Output = Interval;

    fn add(self, rhs: f64) -> Interval {
        Interval { min: self.min + rhs, max: self.max + rhs }
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, rhs: Interval) -> Interval {
        Interval { min: self + rhs.min, max: self + rhs.max }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "min:{:.3} max:{:.3}", self.min, self.max)
    }
}
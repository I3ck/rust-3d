use std::f64;
use std::fmt;
use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};



use traits::{IsMoveable, HasPosition};
use functions::{sqr_dist};

#[derive (PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Eq for Point {}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point::new();
        sqr_dist(&origin, self).partial_cmp(&sqr_dist(&origin, other)).unwrap_or(Ordering::Equal)
    }
}

impl Hash for Point { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
        (self.z as u64).hash(state);
    }
}

impl IsMoveable for Point {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl HasPosition for Point {
    fn new() -> Box<Self> {
        Box::new(Point{x: 0.0, y: 0.0, z: 0.0})
    }

    fn build(x: f64, y: f64, z: f64) -> Box<Self> {
        Box::new(Point{x: x, y: y, z: z})
    }


    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }

    fn set_x(&mut self, val: f64) {
        self.x = val;
    }

    fn set_y(&mut self, val: f64) {
        self.y = val;
    }

    fn set_z(&mut self, val: f64) {
        self.z = val;
    }

    fn clone(&self) -> Point {
        Point { x: self.x, y: self.y, z: self.z }
    }


}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};

use point_3d::Point3D;
use traits::is_3d::Is3D;
use traits::is_normalized_3d::IsNormalized3D;
use traits::has_position_3d::HasPosition3D;
use functions::{sqr_dist_3d};

#[derive (PartialEq, PartialOrd)]
pub struct Norm3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Eq for Norm3D{}
impl Ord for Norm3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point3D::new();
        sqr_dist_3d(&origin, self).partial_cmp(&sqr_dist_3d(&origin, other)).unwrap_or(Ordering::Equal)
    }
}

impl Hash for Norm3D { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
        (self.z as u64).hash(state);
    }
}

impl Is3D for Norm3D {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
    fn clone(&self) -> Self {
        Norm3D {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}

impl IsNormalized3D for Norm3D {
    fn new<P>(p: P) -> Option<Box<Self>> where P: HasPosition3D {
        match p.abs() {
            0.0 => None,
            l => Some(Box::new(Norm3D {
                x: p.x() / l,
                y: p.y() / l,
                z: p.z() / l,
            }))
        }
    }
    fn norm_x() -> Self {
        Norm3D {
            x: 1.0,
            y: 0.0,
            z: 0.0
        }
    }
    fn norm_y() -> Self {
        Norm3D {
            x: 0.0,
            y: 1.0,
            z: 0.0
        }
    }
    fn norm_z() -> Self {
        Norm3D {
            x: 0.0,
            y: 0.0,
            z: 1.0
        }
    }
}

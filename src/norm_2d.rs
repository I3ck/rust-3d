use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};

use point_2d::Point2D;
use traits::is_2d::Is2D;
use traits::is_normalized_2d::IsNormalized2D;
use traits::has_position_2d::HasPosition2D;
use functions::{sqr_dist2D};

#[derive (PartialEq, PartialOrd)]
pub struct Norm2D {
    pub x: f64,
    pub y: f64
}

impl Eq for Norm2D {}
impl Ord for Norm2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point2D::new();
        sqr_dist2D(&origin, self).partial_cmp(&sqr_dist2D(&origin, other)).unwrap_or(Ordering::Equal)
    }
}

impl Hash for Norm2D { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
    }
}

impl Is2D for Norm2D {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn clone(&self) -> Self {
        Norm2D {
            x: self.x,
            y: self.y
        }
    }
}

impl IsNormalized2D for Norm2D {
    fn new<P>(p: P) -> Option<Box<Self>> where P: HasPosition2D {
        match p.abs() {
            0.0 => None,
            l => Some(Box::new(Norm2D {
                x: p.x() / l,
                y: p.y() / l
            }))
        }
    }
    fn norm_x() -> Self {
        Norm2D {
            x: 1.0,
            y: 0.0
        }
    }
    fn norm_y() -> Self {
        Norm2D {
            x: 0.0,
            y: 1.0
        }
    }
}

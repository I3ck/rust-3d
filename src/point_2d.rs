use std::fmt;
use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};


use traits::is_2d::Is2D;
use traits::is_moveable_2d::IsMoveable2D;
use traits::has_position_2d::HasPosition2D;
use traits::has_editable_position_2d::HasEditablePosition2D;
use traits::has_position_3d::HasPosition3D;
use traits::transformable_to_3d::TransFormableTo3D;
use functions::{sqr_dist_2d};

#[derive (PartialEq, PartialOrd)]
pub struct Point2D {
    pub x: f64,
    pub y: f64
}

impl Eq for Point2D {}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point2D::new();
        sqr_dist_2d(&origin, self).partial_cmp(&sqr_dist_2d(&origin, other)).unwrap_or(Ordering::Equal)
    }
}

impl Hash for Point2D { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
    }
}

impl IsMoveable2D for Point2D {
    fn move_by(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
}

impl Is2D for Point2D {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn clone(&self) -> Point2D {
        Point2D { x: self.x, y: self.y }
    }
}

impl HasPosition2D for Point2D {
    fn new() -> Box<Self> {
        Box::new(Point2D{x: 0.0, y: 0.0})
    }

    fn build(x: f64, y: f64) -> Box<Self> {
        Box::new(Point2D{x: x, y: y})
    }

    fn from<P>(&mut self, other: P) where P: HasPosition2D {
        self.x = other.x();
        self.y = other.y();
    }
}

impl HasEditablePosition2D for Point2D {
    fn set_x(&mut self, val: f64) {
        self.x = val;
    }

    fn set_y(&mut self, val: f64) {
        self.y = val;
    }
}

impl TransFormableTo3D for Point2D {
    fn transform_to_3d<P>(&self, z: f64) -> P where
        P: HasPosition3D {
            
        *P::build(self.x, self.y, z)
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

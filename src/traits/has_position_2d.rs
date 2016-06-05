use std::hash::{Hash};

use traits::is_2d::Is2D;

pub trait HasPosition2D : Is2D +  Eq + PartialEq + Ord + PartialOrd + Hash {
    fn new() -> Box<Self>;
    fn build(x: f64, y: f64) -> Box<Self>;
    fn from<P>(&mut self, other: P) where P: HasPosition2D;

    fn normalized(&self) -> Option<Box<Self>> {
        let l = self.abs();
        if l <= 0.0 {
            None
        }
        else {
            Some(Self::build(self.x() / l, self.y() / l))
        }
    }
}

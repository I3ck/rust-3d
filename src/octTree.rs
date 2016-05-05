use point::{Point};

pub struct OctTree {
    pub root: OctNode,
    pub min: Point,
    pub max: Point
}

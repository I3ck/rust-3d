use point::{Point};
use octNode::{OctNode};

pub struct OctTree {
    pub root: OctNode,
    pub min: Point,
    pub max: Point
}

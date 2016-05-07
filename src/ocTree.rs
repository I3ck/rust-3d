use point::{Point};
use ocNode::{OcNode};

pub struct OcTree {
    pub root: OcNode,
    pub min: Point,
    pub max: Point
}

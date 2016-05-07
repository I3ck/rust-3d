use point::{Point};
use pointCloud::{PointCloud};
use ocNode::{OcNode};

pub struct OcTree {
    pub root: OcNode,
    pub min: Point,
    pub max: Point
}

impl OcTree {
    pub fn new(pc: PointCloud) -> Option<OcTree> {
        match pc.bbox() {
            None => None,
            Some((min, max)) => {
                let root = OcNode::new(&min, &max, pc.data);
                Some(OcTree {root: root, min: min, max: max})
            }
        }
    }
}

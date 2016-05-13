use std::collections::HashSet;
use std::iter::IntoIterator;

use point::{Point};
use pointCloud::{PointCloud};
use ocNode::{OcNode};

pub struct OcTree {
    pub root: OcNode,
    pub min: Point,
    pub max: Point
}

impl OcTree {
    pub fn new(mut pc: PointCloud) -> Option<OcTree> {
        match pc.bbox() {
            None => None,
            Some((min, max)) => {
                let mut uniqueData = Vec::new();
                let mut set: HashSet<Point> = pc.data.into_iter().collect();
                uniqueData.extend(set.into_iter());
                let root = OcNode::new(&min, &max, uniqueData);
                Some(OcTree {root: root, min: min, max: max})
            }
        }
    }

    //@todo rewrite or make new method which returns cog instead of stopping recursion
    pub fn collect(&self,  maxdepth: i8) -> PointCloud {
        let mut result = PointCloud::new();
        self.root.collect(0, maxdepth, &mut result);
        return result;
    }
}

use std::collections::HashSet;
use std::iter::IntoIterator;

use point::{Point};
use pointCloud::{PointCloud};
use ocNode::{OcNode};
use traits::{IsTree, IsOcTree};

pub struct OcTree {
    pub root: Option<OcNode>,
    pub min: Point,
    pub max: Point
}

impl IsTree for OcTree {
    fn new() -> OcTree {
        OcTree {
            root: None,
            min: Point::new(),
            max: Point::new()
        }
    }

    fn size(&self) -> usize {
        match self.root {
            None => 0,
            Some(ref node) => node.size()
        }
    }

    fn to_pointcloud(&self) -> PointCloud {
        self.collect(-1)
    }

    fn build(&mut self, pc: PointCloud) -> bool {
        match pc.bbox() {
            None => false,
            Some((min, max)) => {
                let mut uniqueData = Vec::new();
                let mut set: HashSet<Point> = pc.data.into_iter().collect();
                uniqueData.extend(set.into_iter());
                self.root = Some(OcNode::new(&min, &max, uniqueData));
                self.min = min;
                self.max = max;
                true
            }
        }
    }
}

impl IsOcTree for OcTree {
    //@todo rewrite or make new method which returns cog instead of stopping recursion
    fn collect(&self,  maxdepth: i8) -> PointCloud {
        let mut result = PointCloud::new();
        if let Some(ref node) = self.root {
            node.collect(0, maxdepth, &mut result);
        }
        return result;
    }
}

impl OcTree {


}

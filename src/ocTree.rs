use std::collections::HashSet;
use std::iter::IntoIterator;

use point::{Point};
use pointCloud::{PointCloud};
use ocNode::{OcNode};
use traits::{HasPosition, IsTree, IsOcTree};

pub struct OcTree<P> where P: HasPosition {
    pub root: Option<OcNode<P>>,
    pub min: P,
    pub max: P
}

impl<P> IsTree<P> for OcTree<P> where P: HasPosition {
    fn new() -> OcTree<P> {
        OcTree {
            root: None,
            min: P::new(),
            max: P::new()
        }
    }

    fn size(&self) -> usize {
        match self.root {
            None => 0,
            Some(ref node) => node.size()
        }
    }

    fn to_pointcloud(&self) -> PointCloud<P> {
        self.collect(-1)
    }

    fn build(&mut self, pc: PointCloud<P>) -> bool {
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

impl<P> IsOcTree<P> for OcTree<P> where P: HasPosition {
    //@todo rewrite or make new method which returns cog instead of stopping recursion
    fn collect(&self,  maxdepth: i8) -> PointCloud<P> {
        let mut result = PointCloud::new();
        if let Some(ref node) = self.root {
            node.collect(0, maxdepth, &mut result);
        }
        return result;
    }
}

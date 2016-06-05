use std::collections::HashSet;
use std::iter::IntoIterator;

use point_3d::{Point3D};
use point_cloud_3d::{PointCloud3D};
use oc_node::{OcNode};
use traits::has_position_3d::HasPosition3D;
use traits::has_editable_position_3d::HasEditablePosition3D;
use traits::is_tree_3d::IsTree3D;
use traits::is_oc_tree::IsOcTree;

pub struct OcTree<P> where P: HasEditablePosition3D {
    pub root: Option<OcNode<P>>,
    pub min: P,
    pub max: P
}

impl<P> IsTree3D<P> for OcTree<P> where P: HasEditablePosition3D {
    fn new() -> OcTree<P> {
        OcTree {
            root: None,
            min: *P::new(),
            max: *P::new()
        }
    }

    fn size(&self) -> usize {
        match self.root {
            None => 0,
            Some(ref node) => node.size()
        }
    }

    fn to_pointcloud(&self) -> PointCloud3D<P> {
        self.collect(-1)
    }

    fn build(&mut self, pc: PointCloud3D<P>) -> bool {
        match pc.bbox() {
            None => false,
            Some((min, max)) => {
                let mut uniqueData = Vec::new();
                let mut set = HashSet::new();
                for p in pc.data {
                    set.insert(*p);
                }
                //let mut set: HashSet<P> = pc.data.into_iter().unbox().collect();
                uniqueData.extend(set.into_iter());
                self.min = *P::build(min.x, min.y, min.z);
                self.max = *P::build(max.x, max.y, max.z);
                self.root = Some(OcNode::new(&self.min, &self.max, uniqueData));

                true
            }
        }
    }
}

impl<P> IsOcTree<P> for OcTree<P> where P: HasEditablePosition3D {
    //@todo rewrite or make new method which returns cog instead of stopping recursion
    fn collect(&self,  maxdepth: i8) -> PointCloud3D<P> {
        let mut result = PointCloud3D::new();
        if let Some(ref node) = self.root {
            node.collect(0, maxdepth, &mut result);
        }
        return result;
    }
}

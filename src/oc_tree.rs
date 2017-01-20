/*
Copyright 2016 Martin Buck
This file is part of rust-3d.
rust-3d is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rust-3d is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with rust-3d.  If not, see <http://www.gnu.org/licenses/>.
*/

//@todo clean up similar to pc code

use std::collections::HashSet;
use std::iter::IntoIterator;

use point_cloud_3d::{PointCloud3D};
use oc_node::{OcNode};
use traits::is_buildable_nd::IsBuildableND;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_editable_3d::IsEditable3D;
use traits::has_bounding_box_3d::HasBoundingBox3D;
use traits::is_tree_3d::IsTree3D;
use traits::is_oc_tree::IsOcTree;

pub struct OcTree<P> where
    P: IsEditable3D + IsBuildable3D {

    pub root: Option<OcNode<P>>,
    pub min: P,
    pub max: P
}

impl<P> IsTree3D<P> for OcTree<P> where
    P: IsEditable3D + IsBuildableND + IsBuildable3D + Clone {

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

    fn build(&mut self, pc: PointCloud3D<P>) -> bool { //@todo change to result
        match pc.bounding_box() {
            Err(_) => false,
            Ok((min, max)) => {
                let mut unique_data = Vec::new();
                let mut set = HashSet::new();
                for p in pc.data {
                    set.insert(*p);
                }
                //let mut set: HashSet<P> = pc.data.into_iter().unbox().collect();
                unique_data.extend(set.into_iter());
                self.min = *P::build(min.x, min.y, min.z);
                self.max = *P::build(max.x, max.y, max.z);
                self.root = Some(OcNode::new(&self.min, &self.max, unique_data));

                true
            }
        }
    }
}

impl<P> IsOcTree<P> for OcTree<P> where
    P: IsEditable3D + IsBuildableND + IsBuildable3D + Clone {

    //@todo rewrite or make new method which returns cog instead of stopping recursion
    fn collect(&self,  maxdepth: i8) -> PointCloud3D<P> {
        let mut result = PointCloud3D::new();
        if let Some(ref node) = self.root {
            node.collect(0, maxdepth, &mut result);
        }
        result
    }
}

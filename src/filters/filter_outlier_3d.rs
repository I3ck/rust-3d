/*
Copyright 2017 Martin Buck
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

//! FilterOutlier3D, a filter which removes outliers from a 3D point cloud

//@todo make this work for IsRandomAccessible once KdTree supports it
//@todo or even prevent copy once KdTree is referencing
//@todo KdTree needs constructor (best would be to directly build from IsRandomAcc)
//@todo write 2D version once KdTree also supports 2D

use prelude::*;

use kd_tree::KdTree;

#[derive (Debug, PartialEq, PartialOrd, Default, Clone)]
/// FilterOutlier3D, a filter which removes outliers from a 3D point cloud
pub struct FilterOutlier3D<P> where
    P: IsEditable3D + IsBuildableND + IsBuildable3D + Clone + Default { //@todo these can be reduced once KdTree is split properly

    max_distance: Positive,
    tree: KdTree<P>
}

impl<P> FilterOutlier3D<P> where
    P: IsEditable3D + IsBuildableND + IsBuildable3D + Clone + Default { //@todo these can be reduced once KdTree is split properly
    ///Creates a new FilterOutlier3D
    pub fn new(pc: PointCloud3D<P>, max_distance: Positive) -> Result<Self> {
        let mut tree = KdTree { root: None };
        tree.build(pc)?;
        Ok(FilterOutlier3D {max_distance: max_distance, tree: tree})
    }
}

//@todo this impl requires way too many traits for the searched point, make sure "in_sphere()" and similar only require Is3D
impl<P> IsFilter<P> for FilterOutlier3D<P> where
    P: IsEditable3D + IsBuildableND + IsBuildable3D + Clone + Default { //@todo these can be reduced once KdTree is split properly

    fn is_allowed(&self, p: &P) -> bool {
        let pts = self.tree.in_sphere(p, self.max_distance.get());
        pts.len() >= 2 //must find at least 2, since point will find itself
    }
}

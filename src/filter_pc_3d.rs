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

use std::collections::HashSet;

use point_cloud_3d::PointCloud3D;
use view::View;
use traits::is_3d::Is3D;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_editable_3d::IsEditable3D;
use traits::is_filter_3d::IsFilter3D;
use traits::is_filter_pc_3d::IsFilterPC3D;

//@todo untested
//@todo concave hull, convex hull
//@todo add filter trait pc -> pc which these implement?
//@todo might be better to let these work on indices
//@todo especially the or filter is inefficent without usage of indices
//@todo filters could be written for any type or at least for n dimensions?
//@todo rename these to PC filters, and add PointFilters of signature   filter(&Is_2d) -> bool which can then be used in the pc methods

pub struct FilterPC3D<P> where
    P: IsEditable3D + IsBuildable3D {

    pub filter3D: Box<IsFilter3D<P>>
}

impl<P> IsFilterPC3D<P> for FilterPC3D<P> where
    P: IsEditable3D + IsBuildable3D {

    fn filter(&self, pc: &PointCloud3D<P>, view: &mut View) {
        match view {
            &mut View::Full => {
                let mut indices = HashSet::new();
                for (i, p) in pc.data.iter().enumerate() { //@todo could only iterate the indices within the hashset
                    if self.filter3D.is_allowed(p) {
                        indices.insert(i);
                    }
                }
                *view = View::Restricted(indices);
            }
            &mut View::Restricted(ref mut indices) => {
                for (i, p) in pc.data.iter().enumerate() { //@todo could only iterate the indices within the hashset
                    if !self.filter3D.is_allowed(p) {
                        indices.remove(&i);
                    }
                }
            }
        }
    }
}

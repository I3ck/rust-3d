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

//! FilterPC3D, a filter which can transform any IsFilter into an IsFilterPC3D

use std::marker::PhantomData;
use std::collections::HashSet;

use point_cloud_3d::*;
use view::*;
use traits::is_3d::*;
use traits::is_filter::*;
use traits::is_filter_pc_3d::*;

//@todo untested
//@todo concave hull, convex hull

/// FilterPC3D, a filter which can transform any IsFilter into an IsFilterPC3D
pub struct FilterPC3D<F, P> where
    F: IsFilter<P>,
    P: Is3D {

    filter_3d: Box<F>,
    _marker: PhantomData<P>
}

impl<F, P> FilterPC3D<F, P> where
    F: IsFilter<P>,
    P: Is3D {

    pub fn build(filter_3d: F) -> Self {
        FilterPC3D {filter_3d: Box::new(filter_3d), _marker: PhantomData}
    }
}

impl<F, P> IsFilterPC3D<P> for FilterPC3D<F, P> where
    P: Is3D,
    F: IsFilter<P> {

    fn filter(&self, pc: &PointCloud3D<P>, view: &mut View) {
        if pc.len() == 0 {
            *view = View::Full;
            return;
        }
        match view {
            &mut View::Full => {
                let mut indices = HashSet::new();
                for (i, p) in pc.data.iter().enumerate() {
                    let ref tmp = **p; //@todo get rid of this
                    if self.filter_3d.is_allowed(tmp) {
                        indices.insert(i);
                    }
                }
                *view = View::Restricted(indices);
            }
            &mut View::Restricted(ref mut indices) => {
                let mut indices_to_remove = Vec::<usize>::new();
                let max = pc.len() - 1;
                for index in indices.iter() {
                    if *index > max {
                        indices_to_remove.push(*index);
                        continue;
                    }
                    let ref p = *pc.data[*index];
                    if !self.filter_3d.is_allowed(p) {
                        indices_to_remove.push(*index);
                    }
                }

                for index_to_remove in indices_to_remove {
                    indices.remove(&index_to_remove);
                }
            }
        }
    }
}

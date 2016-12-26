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

use std::rc::Rc;
use std::collections::HashSet;

use point_cloud_2d::PointCloud2D;
use traits::is_buildable_2d::IsBuildable2D;
use traits::is_editable_2d::IsEditable2D;

//@todo untested
//@todo concave hull, convex hull
//@todo add filter trait pc -> pc which these implement?
//@todo might be better to let these work on indices
//@todo especially the or filter is inefficent without usage of indices
//@todo filters could be written for any type or at least for n dimensions?

//@todo move
#[derive(Clone)]
pub enum View {
    Full,
    Restricted(HashSet<usize>)
}

impl View {
    fn union(&mut self, other: View) {
        match other {
            View::Full => { *self = other }
            View::Restricted(indices_other) => {
                match self {
                    &mut View::Full => {}
                    &mut View::Restricted(ref mut indices_source) => {
                        *indices_source = indices_source.union(&indices_other).cloned().collect()
                    }
                }
            }
        }
    }
}
///@todo move to traits
pub trait IsFilter2D<P> where
    P: IsEditable2D + IsBuildable2D {

    fn filter(&self, pc: &PointCloud2D<P>, view: &mut View); //@todo could have optional search structures   also define traits for different search structs e.g. trait solely to search in_box_2d
}

pub struct FilterAnd2D<P> where
    P: IsEditable2D + IsBuildable2D {

    pub filters: Vec<Box<IsFilter2D<P>>>
}

pub struct FilterOr2D<P> where
    P: IsEditable2D + IsBuildable2D {

    pub filters: Vec<Box<IsFilter2D<P>>>
}

impl<P> IsFilter2D<P> for FilterAnd2D<P> where
    P: IsEditable2D + IsBuildable2D {

    fn filter(&self, pc: &PointCloud2D<P>, mut view: &mut View) {
        for f in &self.filters {
            f.filter(&pc, &mut view)
        }
    }
}

impl<P> IsFilter2D<P> for FilterOr2D<P> where
    P: IsEditable2D + IsBuildable2D {

    fn filter(&self, pc: &PointCloud2D<P>, mut view: &mut View) {
        let view_initial = view.clone();
        for f in &self.filters {
            let mut view_now = view_initial.clone();
            f.filter(&pc, &mut view_now);
            view.union(view_now);
        }
    }
}

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
pub enum View<T> {
    Full(Rc<T>),
    Restricted(Rc<T>, HashSet<usize>)
} //@todo maybe just drop T altogether, since this might become async anyway

fn merge_views<T>(target: &mut View<T>, source: View<T>) { //@todo as method? //@todo make sure both use the same object
    match source {
        View::Full(_) => return,
        View::Restricted(_, indices_s) => {
            match target {
                &mut View::Full(pc) => { target = &mut source; return }
                &mut View::Restricted(pc, indices_t) => {
                    indices_t.extend(indices_s);
                    target = &mut View::Restricted(pc, indices_t)
                }
            }
        }
    }
}

///@todo move to traits
pub trait IsFilter2D<P> where
    P: IsEditable2D + IsBuildable2D {

    fn filter(&self, pc: &View<PointCloud2D<P>>) -> View<PointCloud2D<P>>; //@todo could have optional search structures   also define traits for different search structs e.g. trait solely to search in_box_2d
}

pub struct FilterAnd<P> where
    P: IsEditable2D + IsBuildable2D {

    pub filters: Vec<Box<IsFilter2D<P>>>
}

pub struct FilterOr<P> where
    P: IsEditable2D + IsBuildable2D {

    pub filters: Vec<Box<IsFilter2D<P>>>
}

impl<P> IsFilter2D<P> for FilterAnd<P> where
    P: IsEditable2D + IsBuildable2D {

    fn filter(&self, pc: &View<PointCloud2D<P>>) -> View<PointCloud2D<P>> {
        let mut result: View<PointCloud2D<P>>;

        // = Restricted::<PointCloud2D<P>>::new();
        for f in &self.filters {
            result = *f.filter(&result)
        }
        Box::new(result)
    }
}

impl<P> IsFilter2D<P> for FilterOr<P> where
    P: IsEditable2D + IsBuildable2D {

    fn filter(&self, pc: &View<PointCloud2D<P>>) -> View<PointCloud2D<P>> {
        let mut result = PointCloud2D::new();
        for f in &self.filters {
            result.consume(*f.filter(pc));
        }
        //@todo remove duplicates
        Box::new(result)
    }
}

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

//! Module containing FilterOrPC2D, a filter to chain multiple 2D filters with the or condition => must pass any filter to pass this filter

use traits::is_2d::*;
use traits::is_filter_pc_2d::*;
use point_cloud_2d::*;
use view::*;

/// FilterOrPC2D, a filter to chain multiple 2D filters with the or condition => must pass any filter to pass this filter
pub struct FilterOrPC2D<P> where
    P: Is2D {

    pub filters: Vec<Box<IsFilterPC2D<P>>>
}

impl<P> IsFilterPC2D<P> for FilterOrPC2D<P> where
    P: Is2D {

    fn filter(&self, pc: &PointCloud2D<P>, mut view: &mut View) {
        let view_initial = view.clone();
        for f in &self.filters {
            let mut view_now = view_initial.clone();
            f.filter(&pc, &mut view_now);
            view.union(view_now);
        }
    }
}

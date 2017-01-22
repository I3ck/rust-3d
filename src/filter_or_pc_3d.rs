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

use traits::is_3d::*;
use traits::is_filter_pc_3d::*;
use point_cloud_3d::*;
use view::*;

pub struct FilterOrPC3D<P> where
    P: Is3D {

    pub filters: Vec<Box<IsFilterPC3D<P>>>
}

impl<P> IsFilterPC3D<P> for FilterOrPC3D<P> where
    P: Is3D {

    fn filter(&self, pc: &PointCloud3D<P>, mut view: &mut View) {
        let view_initial = view.clone();
        for f in &self.filters {
            let mut view_now = view_initial.clone();
            f.filter(&pc, &mut view_now);
            view.union(view_now);
        }
    }
}

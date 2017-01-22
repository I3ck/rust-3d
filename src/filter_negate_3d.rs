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

use traits::is_filter_3d::*;
use traits::is_3d::*;

pub struct FilterNegate3D<F> where
    F: IsFilter3D {

    filter: Box<F>
}

impl<F> FilterNegate3D<F> where
    F: IsFilter3D {

    pub fn build(filter: F) -> Self {
        FilterNegate3D {filter: Box::new(filter)}
    }
}

impl<F> IsFilter3D for FilterNegate3D<F> where
    F: IsFilter3D {

    fn is_allowed(&self, p: &Is3D) -> bool {
        !self.filter.is_allowed(p)
    }
}

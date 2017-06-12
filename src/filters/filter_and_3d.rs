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

//! FilterAnd3D, a filter within 3D space which combines two filters and is true if both of its filters are true

use traits::is_filter_3d::*;
use traits::is_3d::*;
//@todo add tests

/// FilterAND3D, a filter within 3D space which combines two filters and is true if both of its filters is true
pub struct FilterAND3D<F1, F2> where
    F1: IsFilter3D,
    F2: IsFilter3D {

    filter1: Box<F1>,
    filter2: Box<F2>
}

impl<F1, F2> FilterAND3D<F1, F2> where
    F1: IsFilter3D,
    F2: IsFilter3D {

    /// Creates a new FilterAND3D from two other IsFilter3D
    pub fn build(filter1: F1, filter2: F2) -> Self {
        FilterAND3D {filter1: Box::new(filter1), filter2: Box::new(filter2)}
    }
}

impl<F1, F2> IsFilter3D for FilterAND3D<F1, F2> where
    F1: IsFilter3D,
    F2: IsFilter3D {

    fn is_allowed(&self, p: &Is3D) -> bool {
        self.filter1.is_allowed(p) && self.filter2.is_allowed(p)
    }
}

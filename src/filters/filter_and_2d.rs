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

//! Module containing FilterAnd2D, a filter within 2D space which combines two filters and is true if both of its filters are true

use traits::is_filter_2d::*;
use traits::is_2d::*;
///@todo add tests

/// FilterAND2D, a filter within 2D space which combines two filters and is true if both of its filters is true
pub struct FilterAND2D<F1, F2> where
    F1: IsFilter2D,
    F2: IsFilter2D {

    filter1: Box<F1>,
    filter2: Box<F2>
}

impl<F1, F2> FilterAND2D<F1, F2> where
    F1: IsFilter2D,
    F2: IsFilter2D {

    /// Creates a new FilterAND2D from two other IsFilter2D
    pub fn build(filter1: F1, filter2: F2) -> Self {
        FilterAND2D {filter1: Box::new(filter1), filter2: Box::new(filter2)}
    }
}

impl<F1, F2> IsFilter2D for FilterAND2D<F1, F2> where
    F1: IsFilter2D,
    F2: IsFilter2D {

    fn is_allowed(&self, p: &Is2D) -> bool {
        self.filter1.is_allowed(p) && self.filter2.is_allowed(p)
    }
}

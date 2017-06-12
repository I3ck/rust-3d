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

//! FilterAny3D, a filter to chain multiple 3D filters with the or condition => must pass any filter to pass this filter

use traits::is_3d::*;
use traits::is_filter_3d::*;
//@todo add tests

/// FilterAny3D, a filter to chain multiple 3D filters with the or condition => must pass any filter to pass this filter
pub struct FilterAny3D {
    pub filters: Vec<Box<IsFilter3D>>
}

impl IsFilter3D for FilterAny3D {
    fn is_allowed(&self, p: &Is3D) -> bool {
        for f in &self.filters {
            if f.is_allowed(p) {
                return true;
            }
        }
        false
    }
}

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

//! FilterAny, a filter to chain multiple filters with the or condition => must pass any filter to pass this filter

use std::marker::PhantomData;
use traits::is_filter::*;
//@todo add tests

/// FilterAny, a filter to chain multiple filters with the or condition => must pass any filter to pass this filter
pub struct FilterAny<T> {
    pub filters: Vec<Box<IsFilter<T>>>,
    _marker: PhantomData<T>
}

impl<T> FilterAny<T> {
    /// Creates a new FilterAny
    pub fn new() -> Self {
        FilterAny {filters: Vec::new(), _marker: PhantomData}
    }
}

impl<T> IsFilter<T> for FilterAny<T> {
    fn is_allowed(&self, x: &T) -> bool {
        for f in &self.filters {
            if f.is_allowed(x) {
                return true;
            }
        }
        false
    }
}
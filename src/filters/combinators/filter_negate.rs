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

//! FilterNegate, a filter which negates another filter

use std::marker::PhantomData;
use traits::IsFilter;

/// FilterNegate, a filter which negates another filter
pub struct FilterNegate<F, T> where
    F: IsFilter<T> {

    filter: F,
    _marker: PhantomData<T>
}

impl<F, T> FilterNegate<F, T> where
    F: IsFilter<T> {
    /// Creates a new FilterNegate from another IsFilter which will be negated
    pub fn new(filter: F) -> Self {
        FilterNegate {filter, _marker: PhantomData}
    }
}

impl<F, T> IsFilter<T> for FilterNegate<F, T> where
    F: IsFilter<T> {

    fn is_allowed(&self, x: &T) -> bool {
        !self.filter.is_allowed(x)
    }
}

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

//! FilterXOR, a filter which combines two filters and is true iff one of its filters is true

use std::marker::PhantomData;
use traits::IsFilter;

/// FilterXOR, a filter which combines two filters and is true iff one of its filters is true
pub struct FilterXOR<F1, F2, T> where
    F1: IsFilter<T>,
    F2: IsFilter<T> {

    filter1: Box<F1>,
    filter2: Box<F2>,
    _marker: PhantomData<T>
}

impl<F1, F2, T> FilterXOR<F1, F2, T> where
    F1: IsFilter<T>,
    F2: IsFilter<T> {

    /// Creates a new FilterXOR from two other IsFilter
    pub fn new(filter1: F1, filter2: F2) -> Self {
        FilterXOR {filter1: Box::new(filter1), filter2: Box::new(filter2), _marker: PhantomData}
    }
}

impl<F1, F2, T> IsFilter<T> for FilterXOR<F1, F2, T> where
    F1: IsFilter<T>,
    F2: IsFilter<T> {

    fn is_allowed(&self, x: &T) -> bool {
        self.filter1.is_allowed(x) ^ self.filter2.is_allowed(x)
    }
}

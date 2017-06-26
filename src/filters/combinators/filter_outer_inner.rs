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

//! FilterOuterInner, a filter which combines an inner and an outer filter. Where the inner is negated while the outer is allowed. This is useful to create hollow filter shapes

use traits::is_filter::*;
use filters::combinators::filter_and::*;
use filters::combinators::filter_negate::*;
//@todo add tests

/// FilterOuterInner, a filter which combines an inner and an outer filter. Where the inner is negated while the outer is allowed. This is useful to create hollow filter shapes
pub struct FilterOuterInner<FOuter, FInner, T> where
    FOuter: IsFilter<T>,
    FInner: IsFilter<T> {

    filter: FilterAND<FOuter, FilterNegate<FInner, T>, T>
}

impl<FOuter, FInner, T> FilterOuterInner<FOuter, FInner, T> where
    FOuter: IsFilter<T>,
    FInner: IsFilter<T> {

    /// Creates a new FilterOuterInner from two other IsFilter
    pub fn build(filter_outer: FOuter, filter_inner: FInner) -> Self {
        FilterOuterInner {
            filter: FilterAND::new(filter_outer, FilterNegate::new(filter_inner))
        }
    }
}

impl<FOuter, FInner, T> IsFilter<T> for FilterOuterInner<FOuter, FInner, T> where
    FOuter: IsFilter<T>,
    FInner: IsFilter<T> {

    fn is_allowed(&self, x: &T) -> bool {
        self.filter.is_allowed(x)
    }
}

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

//! FilterOuterInner2D, a filter within 2D space which combines an inner and an outer filter. Where the inner is negated while the outer is allowed. This is useful to create hollow filter shapes

use traits::is_filter_2d::*;
use traits::is_2d::*;
use filters::filter_and_2d::*;
use filters::filter_negate_2d::*;
//@todo add tests

/// FilterOuterInner2D, a filter within 2D space which combines an inner and an outer filter. Where the inner is negated while the outer is allowed. This is useful to create hollow filter shapes
pub struct FilterOuterInner2D<FOuter, FInner> where
    FOuter: IsFilter2D,
    FInner: IsFilter2D {

    filter: FilterAND2D<FOuter, FilterNegate2D<FInner>>
}

impl<FOuter, FInner> FilterOuterInner2D<FOuter, FInner> where
    FOuter: IsFilter2D,
    FInner: IsFilter2D {

    /// Creates a new FilterOuterInner2D from two other IsFilter2D
    pub fn build(filter_outer: FOuter, filter_inner: FInner) -> Self {
        FilterOuterInner2D {
            filter: FilterAND2D::build(filter_outer, FilterNegate2D::build(filter_inner))
        }
    }
}

impl<FOuter, FInner> IsFilter2D for FilterOuterInner2D<FOuter, FInner> where
    FOuter: IsFilter2D,
    FInner: IsFilter2D {

    fn is_allowed(&self, p: &Is2D) -> bool {
        self.filter.is_allowed(p)
    }
}

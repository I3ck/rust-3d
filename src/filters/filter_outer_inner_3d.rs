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

//! Module containing FilterOuterInner3D, a filter within 3D space which combines an inner and an outer filter. Where the inner is negated while the outer is allowed. This is useful to create hollow filter shapes

use traits::is_filter_3d::*;
use traits::is_3d::*;
use filters::filter_and_3d::*;
use filters::filter_negate_3d::*;
//@todo add tests

/// FilterOuterInner3D, a filter within 3D space which combines an inner and an outer filter. Where the inner is negated while the outer is allowed. This is useful to create hollow filter shapes
pub struct FilterOuterInner3D<FOuter, FInner> where
    FOuter: IsFilter3D,
    FInner: IsFilter3D {

    filter: FilterAND3D<FOuter, FilterNegate3D<FInner>>
}

impl<FOuter, FInner> FilterOuterInner3D<FOuter, FInner> where
    FOuter: IsFilter3D,
    FInner: IsFilter3D {

    /// Creates a new FilterOuterInner3D from two other IsFilter3D
    pub fn build(filter_outer: FOuter, filter_inner: FInner) -> Self {
        FilterOuterInner3D {
            filter: FilterAND3D::build(filter_outer, FilterNegate3D::build(filter_inner))
        }
    }
}

impl<FOuter, FInner> IsFilter3D for FilterOuterInner3D<FOuter, FInner> where
    FOuter: IsFilter3D,
    FInner: IsFilter3D {

    fn is_allowed(&self, p: &Is3D) -> bool {
        self.filter.is_allowed(p)
    }
}

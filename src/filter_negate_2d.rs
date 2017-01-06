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

use traits::is_filter_2d::IsFilter2D;
use traits::is_2d::Is2D;

pub struct FilterNegate2D<F> where
    F: IsFilter2D {

    filter: Box<F>
}

impl<F> FilterNegate2D<F> where
    F: IsFilter2D {

    pub fn build(filter: F) -> Self {
        FilterNegate2D {filter: Box::new(filter)}
    }
}

impl<F> IsFilter2D for FilterNegate2D<F> where
    F: IsFilter2D {

    fn is_allowed(&self, p: &Is2D) -> bool {
        !self.filter.is_allowed(p)
    }
}

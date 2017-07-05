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

//! FilterAllRandomAccessible, a filter to chain multiple IsFilterRandomAccessible with the and condition => must pass all filters to pass this filter

use traits::is_random_accessible::*;
use traits::is_filter_random_accessible::*;
use view::*;

#[derive (Default)]
/// FilterAllRandomAccessible, a filter to chain multiple IsFilterRandomAccessible with the and condition => must pass all filters to pass this filter
pub struct FilterAllRandomAccessible<RA, T> where
    RA: IsRandomAccessible<T> {

    pub filters: Vec<Box<IsFilterRandomAccessible<RA, T>>>
}

impl<RA, T> IsFilterRandomAccessible<RA, T> for FilterAllRandomAccessible<RA, T> where
    RA: IsRandomAccessible<T> {

    fn filter(&self, ra: &RA, mut view: &mut View) {
        for f in &self.filters {
            f.filter(&ra, &mut view)
        }
    }
}

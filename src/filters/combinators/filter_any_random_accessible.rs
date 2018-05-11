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

//! FilterAnyRandomAccessible, a filter to chain multiple IsFilterRandomAccessible with the or condition => must pass any filter to pass this filter

use prelude::*;

#[derive (Default)]
/// FilterAnyRandomAccessible, a filter to chain multiple IsFilterRandomAccessible with the or condition => must pass any filter to pass this filter
pub struct FilterAnyRandomAccessible<RA, T> where
    RA: IsRandomAccessible<T> {

    pub filters: Vec<Box<IsFilterRandomAccessible<RA, T>>>
}

impl<RA, T> IsFilterRandomAccessible<RA, T> for FilterAnyRandomAccessible<RA, T> where
    RA: IsRandomAccessible<T> {

    fn filter(&self, ra: &RA, view: &mut View) {
        let view_initial = view.clone();
        for f in &self.filters {
            let mut view_now = view_initial.clone();
            f.filter(&ra, &mut view_now);
            view.union(view_now);
        }
    }
}

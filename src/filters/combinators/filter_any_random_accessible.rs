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

use std::marker::PhantomData;

#[derive (Default)]
/// FilterAnyRandomAccessible, a filter to chain multiple IsFilterRandomAccessible with the or condition => must pass any filter to pass this filter
pub struct FilterAnyRandomAccessible<F, RA, T> where
    F: IsFilterRandomAccessible<RA, T>,
    RA: IsRandomAccessible<T> {

    pub filters: Vec<F>,
    _marker_ra: PhantomData<RA>,
    _marker_t: PhantomData<T>
}

impl<F, RA, T> IsFilterRandomAccessible<RA, T> for FilterAnyRandomAccessible<F, RA, T> where
    F: IsFilterRandomAccessible<RA, T>,
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

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

//! FilterRandomAccessible, a filter which can transform any IsFilter into an IsFilterRandomAccessible

use std::marker::PhantomData;
use std::collections::HashSet;

use prelude::*;

/// FilterRandomAccessible, a filter which can transform any IsFilter into an IsFilterRandomAccessible
pub struct FilterRandomAccessible<F, T> where
    F: IsFilter<T> {

    filter: Box<F>,
    _marker: PhantomData<T>
}

impl<F, T> FilterRandomAccessible<F, T> where
    F: IsFilter<T> {

    pub fn new(filter: F) -> Self {
        FilterRandomAccessible {filter: Box::new(filter), _marker: PhantomData}
    }
}

impl<F, T, RA> IsFilterRandomAccessible<RA, T> for FilterRandomAccessible<F, T> where
    F: IsFilter<T>,
    RA: IsRandomAccessible<T>{

    fn filter(&self, ra: &RA, view: &mut View) {
        if ra.len() == 0 {
            *view = View::Full;
            return;
        }
        match view {
            &mut View::Full => {
                let mut indices = HashSet::new();
                let n = ra.len();
                for i in 0..n {
                    if self.filter.is_allowed(&ra[i]) {
                        indices.insert(i);
                    }
                }
                *view = View::Restricted(indices);
            }
            &mut View::Restricted(ref mut indices) => {
                let mut indices_to_remove = Vec::<usize>::new();
                let max = ra.len() - 1;
                for index in indices.iter() {
                    if *index > max {
                        indices_to_remove.push(*index);
                        continue;
                    }
                    if !self.filter.is_allowed(&ra[*index]) {
                        indices_to_remove.push(*index);
                    }
                }

                for index_to_remove in indices_to_remove {
                    indices.remove(&index_to_remove);
                }
            }
        }
    }
}

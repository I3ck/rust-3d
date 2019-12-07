/*
Copyright 2016 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! FilterRandomAccessible, a filter which can transform any IsFilter into an IsFilterRandomAccessible

use std::{collections::HashSet, marker::PhantomData};

use crate::*;

/// FilterRandomAccessible, a filter which can transform any IsFilter into an IsFilterRandomAccessible
pub struct FilterRandomAccessible<F, T>
where
    F: IsFilter<T>,
{
    filter: F,
    _marker: PhantomData<T>,
}

impl<F, T> FilterRandomAccessible<F, T>
where
    F: IsFilter<T>,
{
    pub fn new(filter: F) -> Self {
        FilterRandomAccessible {
            filter,
            _marker: PhantomData,
        }
    }
}

impl<F, T, RA> IsFilterRandomAccessible<RA, T> for FilterRandomAccessible<F, T>
where
    F: IsFilter<T>,
    RA: IsRandomAccessible<T>,
{
    fn filter(&self, ra: &RA, view: &mut View) {
        if ra.len() == 0 {
            *view = View::Full;
            return;
        }
        match view {
            View::Full => {
                let mut indices = HashSet::new();
                let n = ra.len();
                for i in 0..n {
                    if self.filter.is_allowed(&ra[i]) {
                        indices.insert(i);
                    }
                }
                *view = View::Restricted(indices);
            }
            View::Restricted(indices) => {
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

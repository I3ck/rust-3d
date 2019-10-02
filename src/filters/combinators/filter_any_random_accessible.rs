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

//! FilterAnyRandomAccessible, a filter to chain multiple IsFilterRandomAccessible with the or condition => must pass any filter to pass this filter

use crate::prelude::*;

#[derive(Default)]
/// FilterAnyRandomAccessible, a filter to chain multiple IsFilterRandomAccessible with the or condition => must pass any filter to pass this filter
pub struct FilterAnyRandomAccessible<RA, T>
where
    RA: IsRandomAccessible<T>,
{
    pub filters: Vec<Box<dyn IsFilterRandomAccessible<RA, T>>>,
}

impl<RA, T> IsFilterRandomAccessible<RA, T> for FilterAnyRandomAccessible<RA, T>
where
    RA: IsRandomAccessible<T>,
{
    fn filter(&self, ra: &RA, view: &mut View) {
        let view_initial = view.clone();
        for f in &self.filters {
            let mut view_now = view_initial.clone();
            f.filter(&ra, &mut view_now);
            view.union(view_now);
        }
    }
}

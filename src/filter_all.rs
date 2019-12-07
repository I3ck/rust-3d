/*
Copyright 2017 Martin Buck

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

//! FilterAll, a filter to chain multiple filters with the and condition => must pass all filters to pass this filter

use crate::*;
use std::marker::PhantomData;

/// FilterAll, a filter to chain multiple filters with the and condition => must pass all filters to pass this filter
pub struct FilterAll<T> {
    pub filters: Vec<Box<dyn IsFilter<T>>>,
    _marker: PhantomData<T>,
}

impl<T> FilterAll<T> {
    /// Creates a new FilterAll
    pub fn new() -> Self {
        FilterAll {
            filters: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<T> IsFilter<T> for FilterAll<T> {
    fn is_allowed(&self, x: &T) -> bool {
        for f in &self.filters {
            if !f.is_allowed(x) {
                return false;
            }
        }
        true
    }
}

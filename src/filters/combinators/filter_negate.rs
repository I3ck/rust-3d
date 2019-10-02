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

//! FilterNegate, a filter which negates another filter

use std::marker::PhantomData;
use crate::traits::IsFilter;

/// FilterNegate, a filter which negates another filter
pub struct FilterNegate<F, T> where
    F: IsFilter<T> {

    filter: F,
    _marker: PhantomData<T>
}

impl<F, T> FilterNegate<F, T> where
    F: IsFilter<T> {
    /// Creates a new FilterNegate from another IsFilter which will be negated
    pub fn new(filter: F) -> Self {
        FilterNegate {filter, _marker: PhantomData}
    }
}

impl<F, T> IsFilter<T> for FilterNegate<F, T> where
    F: IsFilter<T> {

    fn is_allowed(&self, x: &T) -> bool {
        !self.filter.is_allowed(x)
    }
}

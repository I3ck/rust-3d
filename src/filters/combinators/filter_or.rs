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

//! FilterOR, a filter which combines two filters and is true if one of its filters is true

use crate::traits::IsFilter;
use std::marker::PhantomData;

/// FilterOR, a filter which combines two filters and is true if one of its filters is true
pub struct FilterOR<F1, F2, T>
where
    F1: IsFilter<T>,
    F2: IsFilter<T>,
{
    filter1: F1,
    filter2: F2,
    _marker: PhantomData<T>,
}

impl<F1, F2, T> FilterOR<F1, F2, T>
where
    F1: IsFilter<T>,
    F2: IsFilter<T>,
{
    /// Creates a new FilterOR from two other IsFilter
    pub fn new(filter1: F1, filter2: F2) -> Self {
        FilterOR {
            filter1,
            filter2,
            _marker: PhantomData,
        }
    }
}

impl<F1, F2, T> IsFilter<T> for FilterOR<F1, F2, T>
where
    F1: IsFilter<T>,
    F2: IsFilter<T>,
{
    fn is_allowed(&self, x: &T) -> bool {
        self.filter1.is_allowed(x) || self.filter2.is_allowed(x)
    }
}

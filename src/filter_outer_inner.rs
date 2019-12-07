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

//! FilterOuterInner, a filter which combines an inner and an outer filter. Where the inner is negated while the outer is allowed. This is useful to create hollow filter shapes

use crate::*;

/// FilterOuterInner, a filter which combines an inner and an outer filter. Where the inner is negated while the outer is allowed. This is useful to create hollow filter shapes
pub struct FilterOuterInner<FOuter, FInner, T>
where
    FOuter: IsFilter<T>,
    FInner: IsFilter<T>,
{
    filter: FilterAND<FOuter, FilterNegate<FInner, T>, T>,
}

impl<FOuter, FInner, T> FilterOuterInner<FOuter, FInner, T>
where
    FOuter: IsFilter<T>,
    FInner: IsFilter<T>,
{
    /// Creates a new FilterOuterInner from two other IsFilter
    pub fn new(filter_outer: FOuter, filter_inner: FInner) -> Self {
        FilterOuterInner {
            filter: FilterAND::new(filter_outer, FilterNegate::new(filter_inner)),
        }
    }
}

impl<FOuter, FInner, T> IsFilter<T> for FilterOuterInner<FOuter, FInner, T>
where
    FOuter: IsFilter<T>,
    FInner: IsFilter<T>,
{
    fn is_allowed(&self, x: &T) -> bool {
        self.filter.is_allowed(x)
    }
}

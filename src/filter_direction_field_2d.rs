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

//! FilterDirectionField2D, a filter which can transform any IsDirectionField2D into a filter for (Is2D, IsNormalized2D)

use crate::*;

//------------------------------------------------------------------------------

/// FilterDirectionField2D, a filter which can transform any IsDirectionField2D into a filter for (Is2D, IsNormalized2D)
pub struct FilterDirectionField2D<DF>
where
    DF: IsDirectionField2D,
{
    field: DF,
    eps: Rad,
}

impl<DF> FilterDirectionField2D<DF>
where
    DF: IsDirectionField2D,
{
    pub fn new(field: DF, eps: Rad) -> Self {
        FilterDirectionField2D { field, eps }
    }
}

impl<DF, P, N> IsFilter<(P, N)> for FilterDirectionField2D<DF>
where
    DF: IsDirectionField2D,
    P: Is2D,
    N: IsNormalized2D,
{
    fn is_allowed(&self, pn: &(P, N)) -> bool {
        let expected = self.field.direction_at(&pn.0);
        expected.rad_to(&pn.1) <= self.eps
    }
}

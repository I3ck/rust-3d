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

//! FilterDirectionField3D, a filter which can transform any IsDirectionField3D into a filter for (Is3D, IsNormalized3D)

use prelude::*;

/// FilterDirectionField3D, a filter which can transform any IsDirectionField3D into a filter for (Is3D, IsNormalized3D)
pub struct FilterDirectionField3D<DF> where
    DF: IsDirectionField3D {

    field: DF,
    eps: Rad
}

impl<DF> FilterDirectionField3D<DF> where
    DF: IsDirectionField3D {

    pub fn new(field: DF, eps: Rad ) -> Self {
        FilterDirectionField3D {field, eps}
    }
}

impl<DF, P, N> IsFilter<(P, N)> for FilterDirectionField3D<DF> where
    DF: IsDirectionField3D,
    P:  Is3D,
    N:  IsNormalized3D {

    fn is_allowed(&self, pn: &(P, N)) -> bool {
        let expected = self.field.direction_at(&pn.0);
        return expected.rad_to(&pn.1) <= self.eps;
    }
}
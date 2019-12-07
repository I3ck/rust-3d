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

//! Distances between objects in ND space

use crate::*;

/// Returns the distance between two IsND in case their number of dimensions match
pub fn dist_nd<P, U>(p1: &P, p2: &U) -> Result<f64>
where
    P: IsND,
    U: IsND,
{
    sqr_dist_nd(p1, p2).map(|x| x.sqrt())
}

/// Returns the squared distance between two IsND in case their number of dimensions match
pub fn sqr_dist_nd<P, U>(p1: &P, p2: &U) -> Result<f64>
where
    P: IsND,
    U: IsND,
{
    if P::n_dimensions() != U::n_dimensions() {
        return Err(ErrorKind::DimensionsDontMatch);
    }

    let mut result: f64 = 0.0;
    for i in 0..P::n_dimensions() {
        if let (Ok(val1), Ok(val2)) = (p1.position_nd(i), p2.position_nd(i)) {
            result += (val1 - val2).powi(2);
        } else {
            return Err(ErrorKind::IncorrectDimension);
        }
    }
    Ok(result)
}

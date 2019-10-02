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

//! Distances between objects in 2D space

use crate::prelude::*;

/// Returns the distance between two Is2D
pub fn dist_2d<P, Q>(p: &P, q: &Q) -> f64 where
    P: Is2D,
    Q: Is2D {

    sqr_dist_2d(p, q).sqrt()
}

/// Returns the squared distance between two Is2D
pub fn sqr_dist_2d<P, Q>(p: &P, q: &Q) -> f64 where
    P: Is2D,
    Q: Is2D {

    (p.x() - q.x()).powi(2) + (p.y() - q.y()).powi(2)
}


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

//! Convex hull algorithm returning a Vec of the hull where the points are ordered according to the hull
//! Using Andrew's monotone chain convex hull algorithm https://en.wikibooks.org/wiki/Algorithm_Implementation/Geometry/Convex_hull/Monotone_chain

use crate::*;

/// Convex hull algorithm returning a Vec of the hull where the points are ordered according to the hull
/// Using Andrew's monotone chain convex hull algorithm https://en.wikibooks.org/wiki/Algorithm_Implementation/Geometry/Convex_hull/Monotone_chain
pub fn convex_hull_2d<RA, P>(ra: &RA) -> Vec<P>
where
    RA: IsRandomAccessible<P>,
    P: Is2D + IsBuildable2D + Clone,
{
    let n = ra.len();

    let mut sorted = PointCloud2D::new();
    sorted.append_ra(ra);
    sorted.sort_x();
    let sorted = sorted;

    let mut lower = Vec::<P>::new();
    for i in 0..n {
        while lower.len() >= 2
            && ccw(&lower[lower.len() - 2], &lower[lower.len() - 1], &sorted[i]) <= 0.0
        {
            println!("pop");
            lower.pop().unwrap(); //safe, since only called if len > 0
        }
        lower.push(sorted[i].clone());
    }

    let mut upper = Vec::<P>::new();
    for i in (0..n).rev() {
        while upper.len() >= 2
            && ccw(&upper[upper.len() - 2], &upper[upper.len() - 1], &sorted[i]) <= 0.0
        {
            upper.pop().unwrap(); //safe, since only called if len > 0
        }
        upper.push(sorted[i].clone());
    }

    if lower.len() > 0 {
        lower.pop().unwrap();
    } //safe, since len > 0
    if upper.len() > 0 {
        upper.pop().unwrap();
    } //safe, since len > 0

    let mut result = Vec::<P>::new();
    result.extend(lower);
    result.extend(upper);
    result
}

fn ccw<P1, P2, P3>(p1: &P1, p2: &P2, p3: &P3) -> f64
where
    P1: Is2D,
    P2: Is2D,
    P3: Is2D,
{
    (p2.x() - p1.x()) * (p3.y() - p1.y()) - (p2.y() - p1.y()) * (p3.x() - p1.x())
}

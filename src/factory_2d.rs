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

//! functions used for the creation of 2D shapes and geometries

use std::f64::consts::PI;

use crate::*;

//------------------------------------------------------------------------------

/// Creates a 2D rectangle from given center width and height
pub fn rectangle<P>(center: &P, width: Positive, height: Positive) -> PointCloud2D<P>
where
    P: IsBuildable2D,
{
    let mut pc = PointCloud2D::with_capacity(4);
    let w = width.get();
    let h = height.get();
    pc.push(P::new(center.x() - w / 2.0, center.y() - h / 2.0));
    pc.push(P::new(center.x() + w / 2.0, center.y() - h / 2.0));
    pc.push(P::new(center.x() + w / 2.0, center.y() + h / 2.0));
    pc.push(P::new(center.x() - w / 2.0, center.y() + h / 2.0));
    pc
}

/// Creates a involut circle with the given center, diameter, resolution and start and end angles in radians
pub fn involut_circle<P>(
    center: &P,
    n_points: usize,
    diameter: Positive,
    start: Rad,
    end: Rad,
) -> PointCloud2D<P>
where
    P: IsBuildable2D,
{
    let mut pc = PointCloud2D::with_capacity(n_points);
    let d = diameter.get();
    let p_dist = (end.val - start.val).abs() / (n_points - 1) as f64;

    for i in 0..n_points {
        let current = (i as f64) * p_dist;
        pc.push(P::new(
            center.x() + d / 2.0 * (current.cos() + current * current.sin()),
            center.y() + d / 2.0 * (current.sin() - current * current.cos()),
        ));
    }
    pc
}

/// Creates an arc with the given center, diameter, resolution and start and end angles in radians
pub fn arc<P>(
    center: &P,
    n_points: usize,
    diameter: Positive,
    start: Rad,
    end: Rad,
) -> PointCloud2D<P>
where
    P: IsBuildable2D,
{
    let mut pc = PointCloud2D::with_capacity(n_points);
    let d = diameter.get();
    let p_dist = (end.val - start.val).abs() / (n_points - 1) as f64;

    for i in 0..n_points {
        let radians = start.val + (i as f64) * p_dist;
        pc.push(P::new(
            center.x() + d / 2.0 * radians.cos(),
            center.y() + d / 2.0 * radians.sin(),
        ));
    }
    pc
}

/// Creates an ellipse with the given center, a, b and resolution
pub fn ellipse<P>(center: &P, n_points: usize, ap: Positive, bp: Positive) -> PointCloud2D<P>
where
    P: IsBuildable2D,
{
    let mut pc = PointCloud2D::with_capacity(n_points);
    let p_dist = PI / (n_points - 1) as f64;
    let a = ap.get();
    let b = bp.get();
    let angle: f64 = 0.0;

    for i in 0..n_points {
        let radians = (i as f64) * p_dist;
        pc.push(P::new(
            center.x() + a * radians.cos() * angle.cos() - b * radians.sin() * angle.sin(),
            center.y() + a * radians.cos() * angle.sin() + b * radians.sin() * angle.cos(),
        ));
    }
    pc
}

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

//! interpolations within 2D space. E.g. bezier, linear, cosine

use std::f64::consts::PI;

use prelude::*;

fn factorial(number: usize) -> usize {
    let mut result = 1;

    for i in 1..number+1 {
        result *= i;
    }
    result
}

fn binominal_coefficient(n: usize, k: usize) -> usize {
    factorial(n) / (factorial(k) * factorial(n-k))
}

fn bernstein_polynomial(n: usize, i: usize, t: f64) -> f64 {
    (binominal_coefficient(n, i) as f64) * t.powi(i as i32) * (1.0 - t).powi((n-i) as i32)
}

fn control_polygon<P>(path: &PointCloud2D<P>, n_points: usize, t: f64) -> P where
    P: IsBuildable2D {

    let mut x : f64 = 0.0;
    let mut y : f64 = 0.0;

    for i in 0..n_points+1 {
        let bp = bernstein_polynomial(n_points, i, t);
        x += bp * path.data[i].x();
        y += bp * path.data[i].y();
    }
    P::new(x,y)
}

/// Returns the Bezier interpolation of the given base points
pub fn interpolate_bezier<P>(base_points: &PointCloud2D<P>, n_points: usize) -> Result<PointCloud2D<P>> where
    P: IsBuildable2D {

    if base_points.len() < 2 {
        return Err(ErrorKind::TooFewPoints);
    }

    let mut pc = PointCloud2D::with_capacity(n_points);
    let p_dist = 1.0 / (n_points as f64);

    for i in 0..n_points {
        pc.push(control_polygon(base_points, base_points.len()-1, (i as f64) * p_dist));
    }
    Ok(pc)
}

//@todo function names dont match interpolate vs interpolation...
/// Returns the Cosine interpolation of the given base points
pub fn interpolate_cosine<P>(base_points: &PointCloud2D<P>, n_points: usize) -> Result<PointCloud2D<P>> where
    P : IsBuildable2D {

    if base_points.len() < 2 {
        return Err(ErrorKind::TooFewPoints);
    }

    let mut pc = PointCloud2D::with_capacity(n_points);
    let p_dist = base_points.length() / (n_points - 1) as f64;

    for i in 0..n_points {
        let mut traveled : f64 = 0.0;
        let mut traveled_before : f64 = 0.0;

        for j in 1..base_points.len() {
            let ref p_prev = base_points.data[j-1];
            let ref p_now  = base_points.data[j];

            traveled += ( (p_now.x() - p_prev.x()).powi(2) + (p_now.y() - p_prev.y()).powi(2) ).sqrt();

            if traveled >= p_dist*(i as f64) {
                let proportion = ((i as f64)*p_dist - traveled_before) / (traveled - traveled_before);
                let proportion2 = (1.0 - (proportion*PI).cos() ) / 2.0;
                pc.push(P::new(p_prev.x() + proportion * (p_now.x() - p_prev.x()),
                               p_prev.y() * (1.0 - proportion2) + p_now.y()*proportion2));
                break;
            }
            traveled_before = traveled;
        }
    }
    Ok(pc)
}

/// Returns the linear interpolation of the given base points
pub fn interpolation_linear<P>(base_points: &PointCloud2D<P>, n_points: usize) -> Result<PointCloud2D<P>> where
    P : IsBuildable2D {

    if base_points.len() < 2 {
        return Err(ErrorKind::TooFewPoints);
    }

    let mut pc = PointCloud2D::with_capacity(n_points);
    let p_dist = base_points.length() / (n_points - 1) as f64;

    for i in 0..n_points {
        let mut traveled : f64 = 0.0;
        let mut traveled_before : f64 = 0.0;

        for j in 1..base_points.len() { //@todo fails if path too small, handle this
            let ref p_prev = base_points.data[j-1];
            let ref p_now  = base_points.data[j];

            traveled += ( (p_now.x() - p_prev.x()).powi(2) + (p_now.y() - p_prev.y()).powi(2) ).sqrt();

            if traveled >= p_dist*(i as f64) {
                let proportion = ((i as f64)*p_dist - traveled_before) / (traveled - traveled_before);
                pc.push(P::new(p_prev.x() + proportion * (p_now.x() - p_prev.x()),
                               p_prev.y() + proportion * (p_now.y() - p_prev.y())));
                break;
            }
            traveled_before = traveled;
        }
    }
    Ok(pc)
}

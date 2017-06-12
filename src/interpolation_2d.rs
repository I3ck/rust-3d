/*
Copyright 2016 Martin Buck
This file is part of rust-3d.
rust-3d is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rust-3d is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with rust-3d.  If not, see <http://www.gnu.org/licenses/>.
*/

//! interpolations within 2D space. E.g. bezier, linear, cosine

use std::f64::consts::PI;

use result::*;
use point_cloud_2d::*;
use traits::is_buildable_2d::*;
use traits::has_length::*;


//@todo correct reserving
//@todo some algorithms (e.g. bezier) can be ported to 3d, maybe write them directly generic over the dimension

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
    (binominal_coefficient(n, i) as f64) * t.powi(i as i32) * (1.0 - t).powi((n-i) as i32) //@todo get rid of casts
}

fn control_polygon<P>(path: &PointCloud2D<P>, n_points: usize, t: f64) -> Box<P> where
    P: IsBuildable2D {

    let mut x : f64 = 0.0;
    let mut y : f64 = 0.0;

    //@todo possible bug with +1, c++ version had this, tho. Check again
    for i in 0..n_points+1 {
        let bp = bernstein_polynomial(n_points, i, t);
        x += bp * path.data[i].x();
        y += bp * path.data[i].y();
    }
    P::build(x,y)
}

/// Returns the Bezier interpolation of the given base points
pub fn interpolate_bezier<P>(base_points: &PointCloud2D<P>, n_points: usize) -> Result<Box<PointCloud2D<P>>> where
    P: IsBuildable2D {

    if base_points.len() < 2 {
        return Err(ErrorKind::TooFewPoints);
    }

    let mut pc = PointCloud2D::new();
    let p_dist = 1.0 / (n_points as f64);

    for i in 0..n_points {
        pc.push(*control_polygon(base_points, base_points.len()-1, (i as f64) * p_dist));
    }
    Ok(Box::new(pc))
}

//@todo function names dont match interpolate vs interpolation...
/// Returns the Cosine interpolation of the given base points
pub fn interpolate_cosine<P>(base_points: &PointCloud2D<P>, n_points: usize) -> Result<Box<PointCloud2D<P>>> where
    P : IsBuildable2D {

    if base_points.len() < 2 {
        return Err(ErrorKind::TooFewPoints);
    }

    let mut pc = PointCloud2D::new();
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
                pc.push(*P::build(p_prev.x() + proportion * (p_now.x() - p_prev.x()),
                                  p_prev.y() * (1.0 - proportion2) + p_now.y()*proportion2));
                break;
            }
            traveled_before = traveled;
        }
    }
    Ok(Box::new(pc))
}

/// Returns the linear interpolation of the given base points
pub fn interpolation_linear<P>(base_points: &PointCloud2D<P>, n_points: usize) -> Result<Box<PointCloud2D<P>>> where
    P : IsBuildable2D {

    if base_points.len() < 2 {
        return Err(ErrorKind::TooFewPoints);
    }

    let mut pc = PointCloud2D::new();
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
                pc.push(*P::build(p_prev.x() + proportion * (p_now.x() - p_prev.x()),
                                  p_prev.y() + proportion * (p_now.y() - p_prev.y())));
                break;
            }
            traveled_before = traveled;
        }
    }
    Ok(Box::new(pc))
}

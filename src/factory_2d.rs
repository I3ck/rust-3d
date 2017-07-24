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

//! functions used for the creation of 2D shapes and geometries

use std::f64::consts::PI;

use prelude::*;

/// Creates a 2D rectangle from given center width and height
pub fn rectangle<P>(center: &P, width: Positive, height: Positive) -> Box<PointCloud2D<P>> where
    P: IsBuildable2D {

    let mut pc = PointCloud2D::with_capacity(4);
    let w = width.get();
    let h = height.get();
    pc.push(*P::new(center.x() - w / 2.0, center.y() - h / 2.0));
    pc.push(*P::new(center.x() + w / 2.0, center.y() - h / 2.0));
    pc.push(*P::new(center.x() + w / 2.0, center.y() + h / 2.0));
    pc.push(*P::new(center.x() - w / 2.0, center.y() + h / 2.0));
    Box::new(pc)
}

/// Creates a involut circle with the given center, diameter, resolution and start and end angles in radians
pub fn involut_circle<P>(center: &P, n_points: usize, diameter: Positive, start: Rad, end: Rad) -> Box<PointCloud2D<P>> where
    P: IsBuildable2D {

    let mut pc = PointCloud2D::with_capacity(n_points);
    let d = diameter.get();
    let p_dist = (end.val - start.val).abs() / (n_points - 1) as f64;

    for i in 0..n_points {
        let current = (i as f64) * p_dist;
        pc.push(*P::new(center.x() + d/2.0 * (current.cos() + current * current.sin()),
                        center.y() + d/2.0 * (current.sin() - current * current.cos())));
    }
    Box::new(pc)
}

/// Creates an arc with the given center, diameter, resolution and start and end angles in radians
pub fn arc<P>(center: &P, n_points: usize, diameter: Positive, start: Rad, end: Rad) -> Box<PointCloud2D<P>> where
    P: IsBuildable2D {

    let mut pc = PointCloud2D::with_capacity(n_points);
    let d = diameter.get();
    let p_dist = (end.val - start.val).abs() / (n_points - 1) as f64;

    for i in 0..n_points {
        let radians = start.val + (i as f64) * p_dist;
        pc.push(*P::new(center.x() + d/2.0 * radians.cos(),
                        center.y() + d/2.0 * radians.sin()));
    }
    Box::new(pc)
}

/// Creates an ellipse with the given center, a, b and resolution
pub fn ellipse<P>(center: &P, n_points: usize, ap: Positive, bp: Positive) -> Box<PointCloud2D<P>> where
    P: IsBuildable2D {

    let mut pc = PointCloud2D::with_capacity(n_points);
    let p_dist = PI / (n_points - 1) as f64;
    let a = ap.get();
    let b = bp.get();
    let angle: f64 = 0.0;

    for i in 0..n_points {
        let radians = (i as f64) * p_dist;
        pc.push(*P::new(center.x() + a * radians.cos() * angle.cos() - b * radians.sin() * angle.sin(),
                        center.y() + a * radians.cos() * angle.sin() + b * radians.sin() * angle.cos()));
    }
    Box::new(pc)
}

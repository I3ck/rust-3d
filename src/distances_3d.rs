/*
Copyright 2017 Martin Buck
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

//! Distances between objects in 3D space

use traits::is_3d::*;

/// Returns the distance between two Is3D
pub fn dist_3d(p1: &Is3D, p2: &Is3D) -> f64 {
    sqr_dist_3d(p1,p2).sqrt()
}

/// Returns the squared distance between two Is3D
pub fn sqr_dist_3d(p1: &Is3D, p2: &Is3D) -> f64 {
    (p1.x() - p2.x()).powi(2) + (p1.y() - p2.y()).powi(2) + (p1.z() - p2.z()).powi(2)
}



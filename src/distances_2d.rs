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

//! Distances between objects in 2D space

use prelude::*;

/// Returns the distance between two Is2D
pub fn dist_2d(p1: &Is2D, p2: &Is2D) -> f64 {
    sqr_dist_2d(p1,p2).sqrt()
}

/// Returns the squared distance between two Is2D
pub fn sqr_dist_2d(p1: &Is2D, p2: &Is2D) -> f64 {
    (p1.x() - p2.x()).powi(2) + (p1.y() - p2.y()).powi(2)
}


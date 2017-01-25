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

//! Module containing the IsKdTree3D trait used for KdTrees within 3D space

use result::*;
use traits::is_tree_3d::*;
use traits::is_3d::*;
use point_cloud_3d::*;

/// IsKdTree3D is atrait used for KdTrees within 3D space
pub trait IsKdTree3D<P> : IsTree3D<P> where
    P: Is3D {
    /// Should return the nearest neighbour to search, if there is any
    fn nearest(&self, search: &P) -> Result<P>;
    /// Should return the k nearest neighbours to search
    fn knearest(&self, search: &P, n: usize) -> PointCloud3D<P>;
    /// Should return all positions within a sphere around search
    fn in_sphere(&self, search: &P, radius: f64) -> PointCloud3D<P>;
    /// Should return all positions within a box around search
    fn in_box(&self, search: &P, x_size: f64, y_size: f64, z_size: f64) -> PointCloud3D<P>;
}

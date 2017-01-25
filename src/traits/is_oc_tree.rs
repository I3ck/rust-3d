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

//! Module containing the IsOcTree trait used for OcTrees

use traits::is_tree_3d::*;
use traits::is_3d::*;
use point_cloud_3d::*;

/// IsOcTree is a trait used for OcTrees
pub trait IsOcTree<P> : IsTree3D<P> where
    P: Is3D {
    /// Should return all positions up the given depth
    fn collect(&self, maxdepth: i8) -> PointCloud3D<P>;
}

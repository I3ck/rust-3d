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

//! IsTree3D trait used for types which are any type of tree within 3D space

use traits::is_3d::*;
use point_cloud_3d::*;

/// IsTree3D is a trait used for types which are any type of tree within 3D space
pub trait IsTree3D<P> where
    P: Is3D {
    /// Should create a new tree
    fn new() -> Self;
    /// Should return the number of elements within the tree
    fn size(&self) -> usize;
    /// Should return all positions within the tree as point cloud
    fn to_pointcloud(&self) -> PointCloud3D<P>; //@todo as trait
    /// Should create a new tree from a given point cloud
    fn build(&mut self, pc : PointCloud3D<P>) -> bool; //@todo return Result
}

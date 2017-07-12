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

//! IsVoxelImage trait used for any type of voxel image

use prelude::*;

/// IsVoxelImage is a trait used for any type of voxel image
pub trait IsVoxelImage<T> {
    /// Should return the number of voxels in x-direction
    fn size_x(&self) -> usize;
    /// Should return the number of voxels in y-direction
    fn size_y(&self) -> usize;
    /// Should return the number of voxels in z-direction
    fn size_z(&self) -> usize;
    /// Should return the voxel at a given x y z coordinate
    fn voxel(&self, x: usize, y: usize, z: usize) -> Result<T>;
}

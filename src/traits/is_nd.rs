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

//! Module containing the IsND trait used for types which are positioned within the n-dimensional space

use result::*;

/// IsND is a trait used for types which are positioned within the n-dimensional space
pub trait IsND {
    /// Should return the number of dimensions. E.g. 2 for points in 2D space, 3 for points in 3D space etc.
    fn n_dimensions() -> usize where Self: Sized;
    /// Should return the value of a given dimensions. E.g. for 2D position with x = 4.3, y = 1.8 the result for dimension = 1 should be 1.8
    fn get_position(&self, dimension: usize) -> Result<f64>; //@todo rename
}

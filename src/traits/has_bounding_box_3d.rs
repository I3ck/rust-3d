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

//! HasBoundingBox3D trait for types which might have a bounding box

use prelude::*;

/// HasBoundingBox3D is a trait for types which might have a bounding box
pub trait HasBoundingBox3D {
    /// Should return the bounding box as a pair of two points. The first point should be the minimum for all coordinates, the second the maximum for all coordinates
    fn bounding_box(&self) -> Result<BoundingBox3D>;
}

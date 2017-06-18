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

//! IsSortableND trait used for collections which can be sorted by certain dimensions. E.g. x,y,z

use result::*;

/// IsSortableND trait used for collections which can be sorted by certain dimensions. E.g. x,y,z
pub trait IsSortableND {
    /// Should return the number of dimensions. E.g. 2 for 2D space, 3 for 3D space etc.
    fn n_dimensions() -> usize where Self: Sized;
    /// Should sort all elements by the given dimension
    fn sort_dim(&mut self, dimension: usize) -> Result<()>;
}

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

//! IsMoveable2D trait used for types within 2D space which can be moved

/// IsMoveable2D is a trait used for types within 2D space which can be moved
pub trait IsMoveable2D { //@todo remove trait and impl in IsBuildable2D
    /// Should move the object by the given offset
    fn move_by(&mut self, x: f64, y: f64);
}

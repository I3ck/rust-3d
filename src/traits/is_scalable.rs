/*
Copyright 2018 Martin Buck
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

//! IsScalable trait used for types that can have their size scaled by a factor

use prelude::*;

/// IsScalable trait used for types that can have their size scaled by a factor
pub trait IsScalable {
    /// Should scale by the given factor. 0.5 -> half size, 2.0 double the size without moving the position/center
    fn scale(&mut self, factor: Positive); 
}
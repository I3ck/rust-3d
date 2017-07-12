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

//! Ray2D, a ray within 2D space

use std::fmt;

use prelude::*;

#[derive (PartialEq, PartialOrd, Eq, Clone, Hash)]
/// Ray2D, a ray within 2D space
pub struct Ray2D {
    pub line: Line2D
}

impl IsMovable2D for Ray2D {
    fn move_by(&mut self, x: f64, y: f64) {
        self.line.move_by(x, y);
    }
}

impl fmt::Display for Ray2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.line.fmt(f)
    }
}

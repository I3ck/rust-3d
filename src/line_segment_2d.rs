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

//! LineSegment2D, a line segment within 2D space

//@todo haslength
//@todo has bounding box
//@todo has center of gravity

use std::fmt;

use traits::is_2d::*;
use traits::is_movable_2d::*;

use line_2d::*;
use positive::*;

#[derive (PartialEq, PartialOrd, Eq, Clone, Hash)]
/// LineSegment2D, a line segment within 2D space
pub struct LineSegment2D {
    pub line: Line2D,
    pub length: Positive,
}

impl IsMovable2D for LineSegment2D {
    fn move_by(&mut self, x: f64, y: f64) {
        self.line.move_by(x, y);
    }
}

impl fmt::Display for LineSegment2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {} -> {} * {}, {})", self.line.source.x(), self.line.source.y(), self.length, self.line.dir.x(), self.line.dir.y())
    }
}

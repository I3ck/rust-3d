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

//! Line2D, a line within 2D space

use std::fmt;

use prelude::*;

#[derive (Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// Line2D, a line within 2D space
pub struct Line2D {
    pub anchor: Point2D,
    pub dir: Norm2D
}

impl Line2D {
    /// Creates a new Line2D from an anchor point and a direction
    pub fn new(anchor: Point2D, dir: Norm2D) -> Self {
        Line2D{anchor: anchor, dir: dir}
    }
}

impl IsMovable2D for Line2D {
    fn move_by(&mut self, x: f64, y: f64) {
        self.anchor.move_by(x, y);
    }
}

impl fmt::Display for Line2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {} -> {}, {})", self.anchor.x(), self.anchor.y(), self.dir.x(), self.dir.y())
    }
}

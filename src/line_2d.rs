/*
Copyright 2017 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! Line2D, a line within 2D space

use std::fmt;

use crate::prelude::*;

#[derive (Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// Line2D, a line within 2D space
pub struct Line2D {
    pub anchor: Point2D,
    pub dir: Norm2D
}

impl Line2D {
    /// Creates a new Line2D from an anchor point and a direction
    pub fn new(anchor: Point2D, dir: Norm2D) -> Self {
        Line2D{anchor, dir}
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

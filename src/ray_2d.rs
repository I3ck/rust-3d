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

//! Ray2D, a ray within 2D space

use std::fmt;

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// Ray2D, a ray within 2D space
pub struct Ray2D {
    pub line: Line2D,
}

impl Ray2D {
    /// Creates a new Ray2D from a Line2D
    pub fn new(line: Line2D) -> Self {
        Ray2D { line }
    }
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

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

//! Ray3D, a ray within 3D space

use std::fmt;

use crate::prelude::*;

#[derive (Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// Ray3D, a ray within 3D space
pub struct Ray3D {
    pub line: Line3D
}

impl Ray3D {
    /// Creates a new Ray3D from a Line3D
    pub fn new(line: Line3D) -> Self {
        Ray3D {line}
    }
}

impl IsMovable3D for Ray3D {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.line.move_by(x, y, z);
    }
}

impl fmt::Display for Ray3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.line.fmt(f)
    }
}

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

//! Face3, a face defined by 3 indices (e.g. used for Mesh)

use std::fmt;

use crate::prelude::*;

#[derive (Debug, PartialEq, PartialOrd, Clone, Eq, Ord, Hash)]
/// Face3, a face defined by 3 indices (e.g. used for Mesh)
pub struct Face3 {
    pub a: VId,
    pub b: VId,
    pub c: VId
}

impl Face3 {
    /// Creates a new Face3 from 3 indices
    pub fn new(a: VId, b: VId, c: VId) -> Self {
        Face3 {a, b, c}
    }
}

impl IsTopologyUnit for Face3 {
    fn n_vids() -> usize {
        3
    }

    fn vid(&self, index: usize) -> Result<VId> {
        match index {
            0 => Ok(self.a),
            1 => Ok(self.b),
            2 => Ok(self.c),
            _ => Err(ErrorKind::IncorrectUnitID)
        }
    }
}

impl fmt::Display for Face3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.a.val, self.b.val, self.c.val)
    }
}

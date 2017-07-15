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

//! Face3, a face defined by 3 indices (e.g. used for Mesh)

use std::fmt;

use prelude::*;

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
        Face3 {a: a, b: b, c: c}
    }
}

impl IsTopologyUnit for Face3 {
    fn n_vids() -> usize {
        3
    }

    fn get_vid(&self, index: usize) -> Result<VId> {
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

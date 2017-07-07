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

//! Containing strong type definitions for safer usage

use std::fmt;

//@todo write macro for these

#[derive (Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Vertex ID
pub struct VId {
    pub val: usize
}

impl fmt::Display for VId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[derive (Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Face ID
pub struct FId {
    pub val: usize
}

impl fmt::Display for FId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[derive (Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Edge ID
pub struct EId {
    pub val: usize
}
impl fmt::Display for EId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

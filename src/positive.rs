/*
Copyright 2016 Martin Buck
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

//! Module containing Positive, a wrapper for a f64 value, ensuring it is always > 0

use result::*;

/// Positive, a wrapper for a f64 value, ensuring it is always > 0
pub struct Positive {
    val: f64
}

impl Positive {
    /// Creates a new Positive if input > 0, fails otherwise
    pub fn new(val: f64) -> Result<Positive> {
        if val > 0.0 {
            return Ok(Positive {val: val});
        }
        Err(ErrorKind::NumberInWrongRange)
    }

    /// Returns the wrapped value
    pub fn get(&self) -> f64 {
        self.val
    }
}

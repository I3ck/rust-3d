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

//! IsBuildable3D trait used for types which are positioned in 3D space and can be constructed

extern crate core;

use self::core::str::FromStr;
use std::hash::{Hash};

use prelude::*;

/// IsBuildable3D is a trait used for types which are positioned in 3D space and can be constructed
pub trait IsBuildable3D :
    Sized +
    Is3D +
    Eq +
    PartialEq +
    Ord +
    PartialOrd +
    Hash {

    /// Should build an object from x, y and z coordinates
    fn new(x: f64, y: f64, z: f64) -> Self;
    /// Should use the coordinates of another as its own
    fn from<P>(&mut self, other: P) where
        P: Is3D;

    /// Uses the coordinates of other to create a new
    fn new_from<P>(other: P) -> Self where
        P: Is3D {
            Self::new(other.x(), other.y(), other.z())
    }

    /// Applies a matrix to this
    fn multiply_m(&self, m: &Matrix4) -> Self {
        let x = self.x() * m.data[0][0] + self.y() * m.data[0][1] + self.z() * m.data[0][2] + m.data[0][3];
        let y = self.x() * m.data[1][0] + self.y() * m.data[1][1] + self.z() * m.data[1][2] + m.data[1][3];
        let z = self.x() * m.data[2][0] + self.y() * m.data[2][1] + self.z() * m.data[2][2] + m.data[2][3];
        Self::new(x, y, z)
    }
    /// Returns this with normalized values
    fn normalized(&self) -> Result<Self> {
        let l = self.abs();
        if l.get() == 0.0 {
            return Err(ErrorKind::NormalizeVecWithoutLength);
        }
        Ok(Self::new(self.x() / l.get(), self.y() / l.get(), self.z() / l.get()))
    }
    /// Creates this from a "x y z" string. E.g. "32.2 14.7 1.90"
    fn parse(text: String) -> Result<Self> {
        let split = text.trim().split(" ");
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            3 => {
                let x = f64::from_str(words[0]).map_err(|e| e.to_error_kind())?;
                let y = f64::from_str(words[1]).map_err(|e| e.to_error_kind())?;
                let z = f64::from_str(words[2]).map_err(|e| e.to_error_kind())?;
                Ok(Self::new(x,y,z))
            },
            _ => Err(ErrorKind::ParseError)
        }
    }
}

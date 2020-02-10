/*
Copyright 2016 Martin Buck

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

//! IsBuildable3D trait used for types which are positioned in 3D space and can be constructed

use core::str::FromStr;
use std::hash::Hash;

use crate::*;

/// IsBuildable3D is a trait used for types which are positioned in 3D space and can be constructed
pub trait IsBuildable3D: Sized + Is3D + Eq + PartialEq + Ord + PartialOrd + Hash {
    /// Should build an object from x, y and z coordinates
    fn new(x: f64, y: f64, z: f64) -> Self;
    /// Should use the coordinates of another as its own
    fn from<P>(&mut self, other: &P)
    where
        P: Is3D;

    /// Uses the coordinates of other to create a new
    fn new_from<P>(other: &P) -> Self
    where
        P: Is3D,
    {
        Self::new(other.x(), other.y(), other.z())
    }

    /// Applies a matrix to this
    fn multiply_m(&self, m: &Matrix4) -> Self {
        let x = self.x() * m.data[0][0]
            + self.y() * m.data[0][1]
            + self.z() * m.data[0][2]
            + m.data[0][3];
        let y = self.x() * m.data[1][0]
            + self.y() * m.data[1][1]
            + self.z() * m.data[1][2]
            + m.data[1][3];
        let z = self.x() * m.data[2][0]
            + self.y() * m.data[2][1]
            + self.z() * m.data[2][2]
            + m.data[2][3];
        Self::new(x, y, z)
    }
    /// Returns this with normalized values
    fn normalized(&self) -> Result<Self> {
        let l = self.abs();
        if l.get() == 0.0 {
            return Err(ErrorKind::NormalizeVecWithoutLength);
        }
        Ok(Self::new(
            self.x() / l.get(),
            self.y() / l.get(),
            self.z() / l.get(),
        ))
    }
    /// Returns a new object with 0/0/0 as coordinates
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    /// Creates this from a "x y z" string. E.g. "32.2 14.7 1.90"
    fn parse(text: &str) -> Result<Self> {
        let split = text.trim().split(" ");
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            3 => {
                let x = f64::from_str(words[0]).map_err(|e| e.to_error_kind())?;
                let y = f64::from_str(words[1]).map_err(|e| e.to_error_kind())?;
                let z = f64::from_str(words[2]).map_err(|e| e.to_error_kind())?;
                Ok(Self::new(x, y, z))
            }
            _ => Err(ErrorKind::ParseError),
        }
    }
    /// Returns the center between this and other
    fn center<P>(&self, other: &P) -> Self
    where
        P: Is3D,
    {
        Self::new(
            0.5 * (self.x() + other.x()),
            0.5 * (self.y() + other.y()),
            0.5 * (self.z() + other.z()),
        )
    }
}

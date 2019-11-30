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

//! IsBuildable2D trait used for types which are positioned in 2D space and can be constructed

use core::str::FromStr;
use std::hash::Hash;

use crate::prelude::*;

/// IsBuildable2D is a trait used for types which are positioned in 2D space and can be constructed
pub trait IsBuildable2D:
    Sized + Is2D + IsBuildableND + Eq + PartialEq + Ord + PartialOrd + Hash
{
    /// Should build an object from x and y coordinates
    fn new(x: f64, y: f64) -> Self;
    /// Should use the coordinates of another as its own
    fn from<P>(&mut self, other: &P)
    where
        P: Is2D;

    /// Uses the coordinates of other to create a new
    fn new_from<P>(other: &P) -> Self
    where
        P: Is2D,
    {
        Self::new(other.x(), other.y())
    }
    /// Returns this with normalized values
    fn normalized(&self) -> Result<Self> {
        let l = self.abs();
        if l.get() == 0.0 {
            return Err(ErrorKind::NormalizeVecWithoutLength);
        }
        Ok(Self::new(self.x() / l.get(), self.y() / l.get()))
    }
    /// Returns a new object with 0/0 as coordinates
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    /// Applies a matrix to this
    fn multiply_m(&self, m: &Matrix3) -> Self {
        let x = self.x() * m.data[0][0] + self.y() * m.data[0][1] + m.data[0][2];
        let y = self.x() * m.data[1][0] + self.y() * m.data[1][1] + m.data[1][2];
        Self::new(x, y)
    }
    /// Creates this from a "x y" string. E.g. "4.3 17.29"
    fn parse(text: String) -> Result<Self> {
        let split = text.trim().split(" ");
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            2 => {
                let x = f64::from_str(words[0]).map_err(|e| e.to_error_kind())?;
                let y = f64::from_str(words[1]).map_err(|e| e.to_error_kind())?;
                Ok(Self::new(x, y))
            }
            _ => Err(ErrorKind::ParseError),
        }
    }
}

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

//! IsBuildable2D trait used for types which are positioned in 2D space and can be constructed

extern crate core;

use std::hash::{Hash};
use self::core::str::FromStr;

use prelude::*;

/// IsBuildable2D is a trait used for types which are positioned in 2D space and can be constructed
pub trait IsBuildable2D :
    Sized +
    Is2D +
    IsBuildableND +
    Eq +
    PartialEq +
    Ord +
    PartialOrd +
    Hash {

    /// Should build an object from x and y coordinates
    fn new(x: f64, y: f64) -> Self;

    /// Should use the coordinates of another as its own
    fn from<P>(&mut self, other: P) where
        P: Is2D;

    /// Returns this with normalized values
    fn normalized(&self) -> Result<Self> {
        let l = self.abs();
        if l.get() == 0.0 {
            return Err(ErrorKind::NormalizeVecWithoutLength);
        }
        Ok(Self::new(self.x() / l.get(), self.y() / l.get()))
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
                Ok(Self::new(x,y))
            },
            _ => Err(ErrorKind::ParseError)
        }
    }
}

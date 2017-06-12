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

use result::*;
use traits::is_2d::*;
use traits::is_buildable_nd::*;

/// IsBuildable2D is a trait used for types which are positioned in 2D space and can be constructed
pub trait IsBuildable2D :
    Is2D +
    IsBuildableND +
    Eq +
    PartialEq +
    Ord +
    PartialOrd +
    Hash {

    /// Should build an object from x and y coordinates
    fn build(x: f64, y: f64) -> Box<Self>; //@todo rename both here and in 3d, also possibly rename the trait

    /// Should use the coordinates of another as its own
    fn from<P>(&mut self, other: P) where //@todo only require Is2D for other?
        P: IsBuildable2D;

    /// Returns this with normalized values
    fn normalized(&self) -> Result<Box<Self>> {
        let l = self.abs();
        if l <= 0.0 {
            return Err(ErrorKind::NormalizeVecWithoutLength);
        }
        Ok(Self::build(self.x() / l, self.y() / l))
    }

    /// Creates this from a "x y" string. E.g. "4.3 17.29"
    fn parse(text: String) -> Result<Box<Self>> {
        let split = text.split(" ");
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            2 => {
                let x = f64::from_str(words[0]).map_err(|e| e.to_error_kind())?;
                let y = f64::from_str(words[1]).map_err(|e| e.to_error_kind())?;
                Ok(Self::build(x,y))
            },
            _ => Err(ErrorKind::ParseError)
        }
    }
}

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

//! Module containing the IsBuildable3D trait used for types which are positioned in 3D space and can be constructed

extern crate core;

use self::core::str::FromStr;
use std::hash::{Hash};

use result::*;
use traits::is_3d::*;
use traits::is_buildable_nd::*;
use matrix4::*;

/// IsBuildable3D is a trait used for types which are positioned in 3D space and can be constructed
pub trait IsBuildable3D :
    Is3D +
    Eq +
    PartialEq +
    Ord +
    PartialOrd +
    Hash {

    /// Should build an object from x, y and z coordinates
    fn build(x: f64, y: f64, z: f64) -> Box<Self>;

    /// Should use the coordinates of another as its own
    fn from<P>(&mut self, other: P) where P: IsBuildable3D; //@todo only require Is2D for other?

    //@todo return new or alter self???
    /// Applies a matrix to this
    fn multiply_m(&self, m: &Matrix4) -> Box<Self> {
        let mut result_x = 0.0;
        let mut result_y = 0.0;
        let mut result_z = 0.0;
        for i in 0..4 {
            for j in 0..4 {
                let addition = match j {
                    0 => m.data[i][j] * self.x(),
                    1 => m.data[i][j] * self.y(),
                    _ => m.data[i][j] * self.z()
                };
                match i { //@todo can be simplified
                    0 => {let newx = result_x + addition; result_x = newx;},
                    1 => {let newy = result_y + addition; result_y = newy;},
                    _ => {let newz = result_z + addition; result_z = newz;},
                }
            }
        }
        Self::build(result_x, result_y, result_z)
    }

    /// Returns this with normalized values
    fn normalized(&self) -> Result<Box<Self>> {
        let l = self.abs();
        if l <= 0.0 {
            return Err(ErrorKind::NormalizeVecWithoutLength);
        }
        Ok(Self::build(self.x() / l, self.y() / l, self.z() / l))
    }

    /// Creates this from a "x y z" string. E.g. "32.2 14.7 1.90"
    fn parse(text: String) -> Result<Box<Self>> {
        let split = text.split(" ");
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            3 => {
                let x = f64::from_str(words[0]).map_err(|e| e.to_error_kind())?;
                let y = f64::from_str(words[1]).map_err(|e| e.to_error_kind())?;
                let z = f64::from_str(words[2]).map_err(|e| e.to_error_kind())?;
                Ok(Self::build(x,y,z))
            },
            _ => Err(ErrorKind::ParseError)
        }
    }
}

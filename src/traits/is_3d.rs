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

//! Is3D trait used for types which are positioned within the 3D space

use traits::is_nd::*;

/// Is3D is a trait used for types which are positioned within the 3D space
pub trait Is3D : IsND {
    /// Should return the x-coordinate
    fn x(&self) -> f64;
    /// Should return the y-coordinate
    fn y(&self) -> f64;
    /// Should return the z-coordinate
    fn z(&self) -> f64;

    /// Returns the Position as x,y,z tuple
    fn pos(&self) -> (f64, f64, f64) {
        ( self.x(), self.y(), self.z() )
    }

    /// Calculates the dot product with another Is3D
    fn dot(&self, other: &Is3D) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    /// The absolute / length of this position
    fn abs(&self) -> f64 {
        ((self.x()).powi(2) + (self.y()).powi(2) + (self.z()).powi(2)).sqrt()
    }

    /// Transforms the position in a "x y z" string. E.g. "3.72 5.99 1.01"
    fn to_str(&self) -> String {
        let sx: String = self.x().to_string();
        let sy: String = self.y().to_string();
        let sz: String = self.z().to_string();

        sx + " " + &sy + " " + &sz
    }
}

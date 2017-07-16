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

//! Is2D trait used for types which are positioned within the 2D space

use prelude::*;

/// Is2D is a trait used for types which are positioned within the 2D space
pub trait Is2D : IsND {
    /// Should return the x-coordinate
    fn x(&self) -> f64;
    /// Should return the y-coordinate
    fn y(&self) -> f64;

    /// Returns the Position as x,y tuple
    fn pos(&self) -> (f64, f64) {
        ( self.x(), self.y() )
    }
    /// Calculates the dot product with another Is2D
    fn dot(&self, other: &Is2D) -> f64 {
        self.x() * other.x() + self.y() * other.y()
    }
    /// Calculates the cross product with another Is2D
    fn cross(&self, other: &Is2D) -> f64 {
        self.x() * other.y() - self.y() * other.x()
    }
    /// The absolute / length of this position
    fn abs(&self) -> f64 {
        ((self.x()).powi(2) + (self.y()).powi(2)).sqrt()
    }
    /// Calculates the angle to the other Is2D in radians
    fn rad_to(&self, other: &Is2D) -> Rad {
        Rad{val: (other.y() - self.y()).atan2(other.x() - self.x())}
    }
    /// Transforms the position in a "x y" string. E.g. "3.72 5.99"
    fn to_str(&self) -> String {
        let sx: String = self.x().to_string();
        let sy: String = self.y().to_string();

        sx + " " + &sy
    }
}

impl<P> HasDistanceTo<P> for Is2D where
    P: Is2D {

    fn sqr_distance(&self, other: &P) -> NonNegative {
        NonNegative::new((self.x() - other.x()).powi(2) + (self.y() - other.y()).powi(2)).unwrap()
    }
}

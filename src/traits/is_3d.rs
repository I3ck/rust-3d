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

//! Is3D trait used for types which are positioned within the 3D space

use prelude::*;
use utils::max_f64_3;

use std::f64::consts::PI;

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
    fn abs(&self) -> NonNegative {
        NonNegative::new(((self.x()).powi(2) + (self.y()).powi(2) + (self.z()).powi(2)).sqrt()).unwrap()
    }
    /// Calculates the angle to the other Is3D in radians
    fn rad_to(&self, other: &Is3D) -> Rad {
        if self.abs().get() == 0.0 || other.abs().get() == 0.0 {
            return Rad { val: PI } //@todo consider returning something else on error
        }
        Rad{ val: (self.dot(other) / (self.abs() * other.abs()).get()).acos() }
    }
    /// Transforms the position in a "x y z" string. E.g. "3.72 5.99 1.01"
    fn to_str(&self) -> String {
        let sx: String = self.x().to_string();
        let sy: String = self.y().to_string();
        let sz: String = self.z().to_string();

        sx + " " + &sy + " " + &sz
    }
}

impl<P> HasDistanceTo<P> for Is3D where
    P: Is3D {

    fn sqr_distance(&self, other: &P) -> NonNegative {
        NonNegative::new((self.x() - other.x()).powi(2) + (self.y() - other.y()).powi(2) + (self.z() - other.z()).powi(2)).unwrap()
    }
}

impl HasDistanceTo<BoundingBox3D> for Is3D {
    fn sqr_distance(&self, other: &BoundingBox3D) -> NonNegative {
        let dx = max_f64_3(other.min_p().x() - self.x(), 0.0, self.x() - other.max_p().x());
        let dy = max_f64_3(other.min_p().y() - self.y(), 0.0, self.y() - other.max_p().y());
        let dz = max_f64_3(other.min_p().z() - self.z(), 0.0, self.z() - other.max_p().z());
        NonNegative::new(dx*dx + dy*dy + dz*dz).unwrap()
    }
}

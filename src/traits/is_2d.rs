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

//! Is2D trait used for types which are positioned within the 2D space

use prelude::*;
use utils::max_f64_3;

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
    fn abs(&self) -> NonNegative {
        NonNegative::new(((self.x()).powi(2) + (self.y()).powi(2)).sqrt()).unwrap()
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

impl HasDistanceTo<BoundingBox2D> for Is2D {
    fn sqr_distance(&self, other: &BoundingBox2D) -> NonNegative {
        let dx = max_f64_3(other.min_p().x() - self.x(), 0.0, self.x() - other.max_p().x());
        let dy = max_f64_3(other.min_p().y() - self.y(), 0.0, self.y() - other.max_p().y());
        NonNegative::new(dx*dx + dy*dy).unwrap()
    }
}

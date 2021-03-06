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

//! IsNormalized3D trait used for types which are positioned within the 3D space and normalized

use crate::*;

//------------------------------------------------------------------------------

/// IsNormalized3D is a trait used for types which are positioned within the 3D space and normalized
pub trait IsNormalized3D: Sized + Is3D {
    /// Should construct a new object and only fail if it can't be normalized
    fn new<P>(p: P) -> Result<Self>
    where
        P: Is3D;

    /// Returns a new normalized object which only points in the positive x-Direction
    fn norm_x() -> Self {
        Self::new(Point3D::new(1.0, 0.0, 0.0)).unwrap()
    }
    /// Returns a new normalized object which only points in the positive y-Direction
    fn norm_y() -> Self {
        Self::new(Point3D::new(0.0, 1.0, 0.0)).unwrap()
    }
    /// Returns a new normalized object which only points in the positive z-Direction
    fn norm_z() -> Self {
        Self::new(Point3D::new(0.0, 0.0, 1.0)).unwrap()
    }
    /// Returns a new normalized object which only points in the negative x-Direction
    fn norm_x_neg() -> Self {
        Self::new(Point3D::new(-1.0, 0.0, 0.0)).unwrap()
    }
    /// Returns a new normalized object which only points in the negative y-Direction
    fn norm_y_neg() -> Self {
        Self::new(Point3D::new(0.0, -1.0, 0.0)).unwrap()
    }
    /// Returns a new normalized object which only points in the negative z-Direction
    fn norm_z_neg() -> Self {
        Self::new(Point3D::new(0.0, 0.0, -1.0)).unwrap()
    }
}

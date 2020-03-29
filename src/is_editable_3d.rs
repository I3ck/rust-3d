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

//! IsEditable3D trait used for types which are positioned in 3D space and their position can be changed

use crate::*;

//------------------------------------------------------------------------------

/// IsEditable3D is a trait used for types which are positioned in 3D space and their position can be changed
pub trait IsEditable3D: Is3D + IsEditableND {
    /// Should set the position in x
    fn set_x(&mut self, val: f64);
    /// Should set the position in y
    fn set_y(&mut self, val: f64);
    /// Should set the position in z
    fn set_z(&mut self, val: f64);

    /// Sets the position from x, y and z values
    #[inline(always)]
    fn set_xyz(&mut self, x: f64, y: f64, z: f64) {
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }
    /// Updates the position with x and y values
    #[inline(always)]
    fn set_xy(&mut self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }
    /// Updates the position with x and z values
    #[inline(always)]
    fn set_xz(&mut self, x: f64, z: f64) {
        self.set_x(x);
        self.set_z(z);
    }
    /// Updates the position with y and z values
    #[inline(always)]
    fn set_yz(&mut self, y: f64, z: f64) {
        self.set_y(y);
        self.set_z(z);
    }
    /// Increases distance towards other by factor
    fn increase_distance_to_by<P>(&mut self, other: &P, factor: Positive)
    where
        P: Is3D,
    {
        let x = other.x() + factor.get() * (self.x() - other.x());
        let y = other.y() + factor.get() * (self.y() - other.y());
        let z = other.z() + factor.get() * (self.z() - other.z());

        self.set_xyz(x, y, z);
    }

    /// Adds the coordinates of other onto this. x = x + other.x ...
    fn add<P>(&mut self, other: &P)
    where
        P: Is3D,
    {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        let z = self.z() + other.z();
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    /// Subtracts the coordinates of other from this. x = x - other.x ...
    fn subtract<P>(&mut self, other: &P)
    where
        P: Is3D,
    {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        let z = self.z() - other.z();
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    /// Scales the coordinates by applying a factor to all of them
    fn scale_pos(&mut self, val: f64) {
        let x = val * self.x();
        let y = val * self.y();
        let z = val * self.z();
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }
}

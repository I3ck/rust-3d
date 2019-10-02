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

//! IsEditable2D trait used for types which are positioned in 2D space and their position can be changed

use crate::prelude::*;

/// IsEditable2D is a trait used for types which are positioned in 2D space and their position can be changed
pub trait IsEditable2D: Is2D + IsEditableND {
    /// Should set the position in x
    fn set_x(&mut self, val: f64);
    /// Should set the position in y
    fn set_y(&mut self, val: f64);

    /// Sets the position from x and y values
    fn set_xy(&mut self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }

    /// Increases distance towards other by factor
    fn increase_distance_to_by<P>(&mut self, other: &P, factor: Positive)
    where
        P: Is2D,
    {
        let x = other.x() + factor.get() * (self.x() - other.x());
        let y = other.y() + factor.get() * (self.y() - other.y());

        self.set_xy(x, y);
    }

    /// Adds the coordinates of other onto this. x = x + other.x ...
    fn add<P>(&mut self, other: &P)
    where
        P: Is2D,
    {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        self.set_x(x);
        self.set_y(y);
    }

    /// Subtracts the coordinates of other from this. x = x - other.x ...
    fn subtract<P>(&mut self, other: &P)
    where
        P: Is2D,
    {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        self.set_x(x);
        self.set_y(y);
    }

    /// Scales the coordinates by applying a factor to all of them
    fn scale_pos(&mut self, val: f64) {
        let x = val * self.x();
        let y = val * self.y();
        self.set_x(x);
        self.set_y(y);
    }

    /// Rotates the position around a center ccw for rad radians
    fn rotate<P>(&mut self, r: Rad, center: &P)
    where
        P: Is2D,
    {
        let rad = r.val;
        let newx =
            center.x() + rad.cos() * (self.x() - center.x()) - rad.sin() * (self.y() - center.y());
        let newy =
            center.y() + rad.sin() * (self.x() - center.x()) + rad.cos() * (self.y() - center.y());

        self.set_x(newx);
        self.set_y(newy);
    }
}

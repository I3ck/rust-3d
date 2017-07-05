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

//! IsEditable2D trait used for types which are positioned in 2D space and their position can be changed

use traits::is_2d::*;
use traits::is_editable_nd::*;

/// IsEditable2D is a trait used for types which are positioned in 2D space and their position can be changed
pub trait IsEditable2D : Is2D + IsEditableND {
    /// Should set the position in x
    fn set_x(&mut self, val: f64); //@todo these kinda make it movable, maybe put into IsMovable3D? Or remove movable trait
    /// Should set the position in y
    fn set_y(&mut self, val: f64);

    /// Sets the position from a x and y values
    fn set_pos(&mut self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }

    /// Adds the coordinates of other onto this. x = x + other.x ...
    fn add<P>(&mut self, other: &P) where
        P: Is2D {

        let x = self.x() + other.x();
        let y = self.y() + other.y();
        self.set_x(x);
        self.set_y(y);
    }

    /// Substracts the coordinates of other from this. x = x - other.x ...
    fn substract<P>(&mut self, other: &P) where
        P: Is2D {

        let x = self.x() - other.x();
        let y = self.y() - other.y();
        self.set_x(x);
        self.set_y(y);
    }

    /// Scales the coordinates by applying a factor to all of them
    fn scale(&mut self, val: f64) {
        let x = val * self.x();
        let y = val * self.y();
        self.set_x(x);
        self.set_y(y);
    }

    /// Rotates the position around a center ccw for rad radians
    fn rotate<P>(&mut self, rad: f64, center: &P) where
        P: Is2D {

        let newx = center.x() + rad.cos() * (self.x() - center.x()) - rad.sin() * (self.y() - center.y());
        let newy = center.y() + rad.sin() * (self.x() - center.x()) + rad.cos() * (self.y() - center.y());

        self.set_x(newx);
        self.set_y(newy);
    }
}

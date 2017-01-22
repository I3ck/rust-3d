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

use traits::is_3d::*;
use traits::is_editable_nd::*;

pub trait IsEditable3D : Is3D + IsEditableND {
    fn set_x(&mut self, val: f64); //@todo these kinda make it moveable, maybe put into IsMoveable3D? Or remove moveable trait

    fn set_y(&mut self, val: f64);

    fn set_z(&mut self, val: f64);

    fn set_pos(&mut self, x: f64, y: f64, z: f64) {
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    fn add<P>(&mut self, other: &P) where
        P: Is3D {

        let x = self.x() + other.x();
        let y = self.y() + other.y();
        let z = self.z() + other.z();
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    fn substract<P>(&mut self, other: &P) where
        P: Is3D {

        let x = self.x() - other.x();
        let y = self.y() - other.y();
        let z = self.z() - other.z();
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    fn scale(&mut self, val: f64) {
        let x = val * self.x();
        let y = val * self.y();
        let z = val * self.z();
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }
}

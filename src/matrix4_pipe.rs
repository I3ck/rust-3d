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

//! Matrix4Pipe, which makes it easier to pipe different matrices in a defined order

use strong_types::*;
use matrix4::*;
use traits::is_buildable_3d::*;

#[derive (Default)]
/// Matrix4Pipe, which makes it easier to pipe different matrices in a defined order
pub struct Matrix4Pipe {
    pub mtranslation: Matrix4,
    pub mrotation: Matrix4,
    pub mscale: Matrix4,
    pub mperspective: Matrix4,
    pub mcamtrans: Matrix4,
    pub mcamlook: Matrix4
}

impl Matrix4Pipe {
    //@todo might be inversed order
    //@todo better overload operator * for Matrix4 to gain nicer syntax
    /// Creates a new matrix as a result of all defined operations set within the pipe
    pub fn result(&self) -> Matrix4 {
        self.mperspective
            .multiply_m(&self.mcamlook
                .multiply_m(&self.mcamtrans
                    .multiply_m(&self.mtranslation
                        .multiply_m(&self.mrotation
                            .multiply_m(&self.mscale)))))
    }
    /// Adds a translation to the pipe
    pub fn add_translation(&mut self, x: f64, y: f64, z: f64) {
        self.mtranslation = Matrix4::translation(x, y, z);
    }
    /// Removes any translation from the pipe
    pub fn remove_translation(&mut self) {
        self.mtranslation = Matrix4::default();
    }

    /// Adds a rotation to the pipe
    pub fn add_rotation(&mut self, x: Rad, y: Rad, z: Rad) {
        self.mrotation = Matrix4::rotation(x, y, z);
    }
    /// Adds a rotation around an axis to the pipe
    pub fn add_rotation_axis<P>(&mut self, axis: &P, rad: Rad) -> bool where P: IsBuildable3D {
        match Matrix4::rotation_axis(axis, rad) {
            Err(_) => return false,
            Ok(m) => { self.mrotation = m; return true; }
        }
    }
    /// Removes any rotation from the pipe
    pub fn remove_rotation(&mut self) {
        self.mrotation = Matrix4::default();
    }

    /// Adds scaling to the pipe
    pub fn add_scale(&mut self, x: f64, y: f64, z: f64) {
        self.mscale = Matrix4::scale(x, y, z);
    }
    /// Removes any scaling from the pipe
    pub fn remove_scale(&mut self) {
        self.mscale = Matrix4::default();
    }

    /// Adds a perspective transformation to the pipe
    pub fn add_perspective(&mut self, close: f64, away: f64, rad: Rad) {
        self.mperspective = Matrix4::perspective(close, away, rad);
    }
    /// Removes any perspective transformation from the pipe
    pub fn remove_perspective(&mut self) {
        self.mperspective = Matrix4::default();
    }

    /// Adds camera translation to the pipe
    pub fn add_camera_translation(&mut self, x: f64, y: f64, z: f64) {
        self.mcamtrans = Matrix4::translation(-x, -y, -z);
    }
    /// Removes any camera translation from the pipe
    pub fn remove_camera_translation(&mut self) {
        self.mcamtrans = Matrix4::default();
    }

    /// Adds a look at target to the pipe
    pub fn add_look_at<P>(&mut self, target: &P, up: &P) -> bool where P: IsBuildable3D {
        match Matrix4::look_at(target, up) {
            Err(_) => return false,
            Ok(m) => { self.mcamlook = m; return true; }
        }
    }
    /// Removes any look at target from the pipe
    pub fn remove_look_at(&mut self) {
        self.mcamlook = Matrix4::default();
    }
}

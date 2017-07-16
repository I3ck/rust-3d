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

use prelude::*;

#[derive (Default, Debug, PartialEq, PartialOrd, Clone)]
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
    /// Creates a new matrix as a result of all defined operations set within the pipe
    pub fn result(&self) -> Matrix4 {
              self.mperspective.clone()
            * self.mcamlook.clone()
            * self.mcamtrans.clone()
            * self.mtranslation.clone()
            * self.mrotation.clone()
            * self.mscale.clone()
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
    pub fn add_rotation_axis<N>(&mut self, axis: &N, rad: Rad) where
        N: IsNormalized3D {

        self.mrotation = Matrix4::rotation_axis(axis, rad)
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
    pub fn add_look_at<P, N>(&mut self, target: &P, up: &N) -> Result<()> where
        P: IsBuildable3D,
        N: IsNormalized3D {

        self.mcamlook = Matrix4::look_at(target, up)?;
        Ok(())
    }
    /// Removes any look at target from the pipe
    pub fn remove_look_at(&mut self) {
        self.mcamlook = Matrix4::default();
    }
}

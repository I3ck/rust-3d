/*
Copyright 2017 Martin Buck
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

//! Matrix3Pipe, which makes it easier to pipe different matrices in a defined order

use prelude::*;

#[derive (Default, Debug, PartialEq, PartialOrd, Clone)]
/// Matrix3Pipe, which makes it easier to pipe different matrices in a defined order
pub struct Matrix3Pipe {
    pub mtranslation: Matrix3,
    pub mrotation: Matrix3,
    pub mscale: Matrix3
}

impl Matrix3Pipe {
    //@todo might be inversed order
    /// Creates a new matrix as a result of all defined operations set within the pipe
    pub fn result(&self) -> Matrix3 {
              self.mtranslation.clone()
            * self.mrotation.clone()
            * self.mscale.clone()
    }
    /// Adds a translation to the pipe
    pub fn add_translation(&mut self, x: f64, y: f64) {
        self.mtranslation = Matrix3::translation(x, y);
    }
    /// Removes any translation from the pipe
    pub fn remove_translation(&mut self) {
        self.mtranslation = Matrix3::default();
    }
    
    /// Adds a rotation to the pipe
    pub fn add_rotation(&mut self, rad: Rad) {
        self.mrotation = Matrix3::rotation(rad);
    }
    /// Removes any rotation from the pipe
    pub fn remove_rotation(&mut self) {
        self.mrotation = Matrix3::default();
    }

    /// Adds scaling to the pipe
    pub fn add_scale(&mut self, x: f64, y: f64) {
        self.mscale = Matrix3::scale(x, y);
    }
    /// Removes any scaling from the pipe
    pub fn remove_scale(&mut self) {
        self.mscale = Matrix3::default();
    }
}

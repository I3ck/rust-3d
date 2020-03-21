/*
Copyright 2017 Martin Buck

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

//! Matrix3Pipe, which makes it easier to pipe different matrices in a defined order

use crate::*;

//------------------------------------------------------------------------------

#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
/// Matrix3Pipe, which makes it easier to pipe different matrices in a defined order
pub struct Matrix3Pipe {
    pub mtranslation: Matrix3,
    pub mrotation: Matrix3,
    pub mscale: Matrix3,
}

impl Matrix3Pipe {
    /// Creates a new matrix as a result of all defined operations set within the pipe
    pub fn result(&self) -> Matrix3 {
        &self.mtranslation * &self.mrotation * &self.mscale
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

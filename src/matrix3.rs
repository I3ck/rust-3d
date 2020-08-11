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

//! Matrix3, a matrix with 3 rows and columns

use std::ops::Mul;

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// Matrix3, a matrix with 3 rows and columns
pub struct Matrix3 {
    pub data: [[f64; 3]; 3],
}

impl Matrix3 {
    /// Creates a new identity matrix
    pub fn identity() -> Matrix3 {
        Matrix3 {
            data: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
    /// Creates a new matrix which contains only zeroes
    pub fn zeroes() -> Matrix3 {
        Matrix3 {
            data: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        }
    }
    /// Creates a new matrix which applies translation
    pub fn translation(x: f64, y: f64) -> Matrix3 {
        Matrix3 {
            data: [[1.0, 0.0, x], [0.0, 1.0, y], [0.0, 0.0, 1.0]],
        }
    }
    /// Creates a new matrix which applies scaling
    pub fn scale(x: f64, y: f64) -> Matrix3 {
        Matrix3 {
            data: [[x, 0.0, 0.0], [0.0, y, 0.0], [0.0, 0.0, 1.0]],
        }
    }
    /// Creates a new matrix which applies rotation
    pub fn rotation(rad: Rad) -> Matrix3 {
        Matrix3 {
            data: [
                [rad.0.cos(), -rad.0.sin(), 0.0],
                [rad.0.sin(), rad.0.cos(), 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Default for Matrix3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Matrix3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Matrix3::default();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = self.data[i][0] * other.data[0][j]
                    + self.data[i][1] * other.data[1][j]
                    + self.data[i][2] * other.data[2][j];
            }
        }
        result
    }
}

impl Mul<&Matrix3> for Matrix3 {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        let mut result = Matrix3::default();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = self.data[i][0] * other.data[0][j]
                    + self.data[i][1] * other.data[1][j]
                    + self.data[i][2] * other.data[2][j];
            }
        }
        result
    }
}

impl Mul for &Matrix3 {
    type Output = Matrix3;

    fn mul(self, other: Self) -> Matrix3 {
        let mut result = Matrix3::default();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = self.data[i][0] * other.data[0][j]
                    + self.data[i][1] * other.data[1][j]
                    + self.data[i][2] * other.data[2][j];
            }
        }
        result
    }
}

impl Mul<f64> for Matrix3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        let mut result = Matrix3::default();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = other * self.data[i][j];
            }
        }
        result
    }
}

impl Mul<f64> for &Matrix3 {
    type Output = Matrix3;

    fn mul(self, other: f64) -> Matrix3 {
        let mut result = Matrix3::default();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = other * self.data[i][j];
            }
        }
        result
    }
}

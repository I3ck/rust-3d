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

//! Matrix3, a matrix with 3 rows and columns

use std::ops::Mul;

use prelude::*;

#[derive (Debug, PartialEq, PartialOrd, Clone)]
/// Matrix3, a matrix with 3 rows and columns
pub struct Matrix3 {
    pub data: [[f64; 3]; 3]
}

impl Matrix3 {
    /// Creates a new matrix which contains only zeroes
    pub fn zeroes() -> Matrix3 {
        Matrix3{
            data: [
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0]
            ]
        }
    }
    /// Creates a new matrix which applies translation
    pub fn translation(x: f64, y: f64) -> Matrix3 {
        Matrix3{
            data: [
                [1.0, 0.0, x],
                [0.0, 1.0, y],
                [0.0, 0.0, 1.0]
            ]
        }
    }
    /// Creates a new matrix which applies scaling
    pub fn scale(x: f64, y: f64) -> Matrix3 {
        Matrix3{
            data: [
                [x,   0.0, 0.0],
                [0.0, y,   0.0],
                [0.0, 0.0, 1.0]
            ]
        }
    }
    /// Creates a new matrix which applies rotation
    pub fn rotation(rad: Rad) -> Matrix3 {
        Matrix3{
            data: [
                [rad.val.cos(), -rad.val.sin(), 0.0],
                [rad.val.sin(),  rad.val.cos(), 0.0],
                [0.0,            0.0,           1.0]
            ]
        }
    }
}

impl Default for Matrix3 {
    fn default() -> Self {
        Matrix3{
            data: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ]
        }
    }
}

impl Mul for Matrix3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Matrix3::default();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] =
                    self.data[i][0] * other.data[0][j] + 
                    self.data[i][1] * other.data[1][j] +
                    self.data[i][2] * other.data[2][j];
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

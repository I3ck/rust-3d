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

//! Matrix4, a matrix with 4 rows and columns

use std::ops::Mul;

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// Matrix4, a matrix with 4 rows and columns
pub struct Matrix4 {
    pub data: [[f64; 4]; 4],
}

impl Matrix4 {
    /// Creates a new identity matrix
    pub fn identity() -> Matrix4 {
        Matrix4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    /// Creates a new matrix which contains only zeroes
    pub fn zeroes() -> Matrix4 {
        Matrix4 {
            data: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }
    /// Creates a new matrix which applies translation
    pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4 {
            data: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    /// Creates a new matrix which applies scaling
    pub fn scale(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4 {
            data: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    /// Creates a new matrix which applies rotation
    pub fn rotation(x: Rad, y: Rad, z: Rad) -> Matrix4 {
        let (mut mx, mut my, mut mz) = (Matrix4::default(), Matrix4::default(), Matrix4::default());
        let (rad_x, rad_y, rad_z) = (x.val, y.val, z.val);

        mx.data[0][0] = 1.0;
        mx.data[0][1] = 0.0;
        mx.data[0][2] = 0.0;
        mx.data[0][3] = 0.0;
        mx.data[1][0] = 0.0;
        mx.data[1][1] = rad_x.cos();
        mx.data[1][2] = -rad_x.sin();
        mx.data[1][3] = 0.0;
        mx.data[2][0] = 0.0;
        mx.data[2][1] = rad_x.sin();
        mx.data[2][2] = rad_x.cos();
        mx.data[2][3] = 0.0;
        mx.data[3][0] = 0.0;
        mx.data[3][1] = 0.0;
        mx.data[3][2] = 0.0;
        mx.data[3][3] = 1.0;

        my.data[0][0] = rad_y.cos();
        my.data[0][1] = 0.0;
        my.data[0][2] = rad_y.sin();
        my.data[0][3] = 0.0;
        my.data[1][0] = 0.0;
        my.data[1][1] = 1.0;
        my.data[1][2] = 0.0;
        my.data[1][3] = 0.0;
        my.data[2][0] = -rad_y.sin();
        my.data[2][1] = 0.0;
        my.data[2][2] = rad_y.cos();
        my.data[2][3] = 0.0;
        my.data[3][0] = 0.0;
        my.data[3][1] = 0.0;
        my.data[3][2] = 0.0;
        my.data[3][3] = 1.0;

        mz.data[0][0] = rad_z.cos();
        mz.data[0][1] = -rad_z.sin();
        mz.data[0][2] = 0.0;
        mz.data[0][3] = 0.0;
        mz.data[1][0] = rad_z.sin();
        mz.data[1][1] = rad_z.cos();
        mz.data[1][2] = 0.0;
        mz.data[1][3] = 0.0;
        mz.data[2][0] = 0.0;
        mz.data[2][1] = 0.0;
        mz.data[2][2] = 1.0;
        mz.data[2][3] = 0.0;
        mz.data[3][0] = 0.0;
        mz.data[3][1] = 0.0;
        mz.data[3][2] = 0.0;
        mz.data[3][3] = 1.0;

        mx * my * mz
    }
    /// Creates a new matrix which applies rotation around an axis
    pub fn rotation_axis<N>(axis: &N, r: Rad) -> Matrix4
    where
        N: IsNormalized3D,
    {
        let rad = r.val;
        let ref u = axis;
        let mut result = Matrix4::default();
        result.data[0][0] = rad.cos() + u.x() * u.x() * (1.0 - rad.cos());
        result.data[0][1] = u.x() * u.y() * (1.0 - rad.cos()) - u.z() * rad.sin();
        result.data[0][2] = u.x() * u.z() * (1.0 - rad.cos()) + u.y() * rad.sin();
        result.data[0][3] = 0.0;
        result.data[1][0] = u.y() * u.x() * (1.0 - rad.cos()) + u.z() * rad.sin();
        result.data[1][1] = rad.cos() + u.y() * u.y() * (1.0 - rad.cos());
        result.data[1][2] = u.y() * u.z() * (1.0 - rad.cos()) - u.x() * rad.sin();
        result.data[1][3] = 0.0;
        result.data[2][0] = u.z() * u.x() * (1.0 - rad.cos()) - u.y() * rad.sin();
        result.data[2][1] = u.z() * u.y() * (1.0 - rad.cos()) + u.x() * rad.sin();
        result.data[2][2] = rad.cos() + u.z() * u.z() * (1.0 - rad.cos());
        result.data[2][3] = 0.0;
        result.data[3][0] = 0.0;
        result.data[3][1] = 0.0;
        result.data[3][2] = 0.0;
        result.data[3][3] = 1.0;
        result
    }
    /// Creates a new matrix which applies perspective transformation
    pub fn perspective(close: f64, away: f64, fov: Rad) -> Matrix4 {
        let fov_rad = fov.val;
        let range = close - away;
        let tan_fov_half = (fov_rad / 2.0).tan();

        let mut result = Matrix4::default();
        result.data[0][0] = 1.0 / (tan_fov_half * away);
        result.data[0][1] = 0.0;
        result.data[0][2] = 0.0;
        result.data[0][3] = 0.0;
        result.data[1][0] = 0.0;
        result.data[1][1] = 1.0 / tan_fov_half;
        result.data[1][2] = 0.0;
        result.data[1][3] = 0.0;
        result.data[2][0] = 0.0;
        result.data[2][1] = 0.0;
        result.data[2][2] = (-close - away) / range;
        result.data[2][3] = 2.0 * away * close / range;
        result.data[3][0] = 0.0;
        result.data[3][1] = 0.0;
        result.data[3][2] = 1.0;
        result.data[3][3] = 1.0;
        result
    }
    /// Creates a new matrix which applies a look at transformation
    pub fn look_at<P, N>(target: &P, up: &N) -> Result<Matrix4>
    where
        P: IsBuildable3D,
        N: IsNormalized3D,
    {
        let n = target.normalized()?;
        let u = cross(&*up, target);
        let v = cross(&n, &u);

        let mut result = Matrix4::default();
        result.data[0][0] = u.x();
        result.data[0][1] = u.y();
        result.data[0][2] = u.z();
        result.data[0][3] = 0.0;
        result.data[1][0] = v.x();
        result.data[1][1] = v.y();
        result.data[1][2] = v.z();
        result.data[1][3] = 0.0;
        result.data[2][0] = n.x();
        result.data[2][1] = n.y();
        result.data[2][2] = n.z();
        result.data[2][3] = 0.0;
        result.data[3][0] = 0.0;
        result.data[3][1] = 0.0;
        result.data[3][2] = 0.0;
        result.data[3][3] = 1.0;
        Ok(result)
    }
}

//------------------------------------------------------------------------------

impl Default for Matrix4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Matrix4::default();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][0] * other.data[0][j]
                    + self.data[i][1] * other.data[1][j]
                    + self.data[i][2] * other.data[2][j]
                    + self.data[i][3] * other.data[3][j];
            }
        }
        result
    }
}

impl Mul<&Matrix4> for Matrix4 {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        let mut result = Matrix4::default();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][0] * other.data[0][j]
                    + self.data[i][1] * other.data[1][j]
                    + self.data[i][2] * other.data[2][j]
                    + self.data[i][3] * other.data[3][j];
            }
        }
        result
    }
}

impl Mul for &Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Self) -> Matrix4 {
        let mut result = Matrix4::default();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[i][0] * other.data[0][j]
                    + self.data[i][1] * other.data[1][j]
                    + self.data[i][2] * other.data[2][j]
                    + self.data[i][3] * other.data[3][j];
            }
        }
        result
    }
}

impl Mul<f64> for Matrix4 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        let mut result = Matrix4::default();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = other * self.data[i][j];
            }
        }
        result
    }
}

impl Mul<f64> for &Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: f64) -> Matrix4 {
        let mut result = Matrix4::default();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = other * self.data[i][j];
            }
        }
        result
    }
}

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

//! Norm3D, a normalized vector within 3D space

use std::{
    cmp::{Eq, Ordering},
    hash::{Hash, Hasher},
    ops::{Mul, Neg},
};

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// Norm3D, a normalized vector within 3D space
pub struct Norm3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Eq for Norm3D {}

impl Ord for Norm3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point3D::default();
        sqr_dist_3d(&origin, self)
            .partial_cmp(&sqr_dist_3d(&origin, other))
            .unwrap_or(Ordering::Equal)
    }
}

impl Hash for Norm3D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_f64(self.x, state);
        hash_f64(self.y, state);
        hash_f64(self.z, state);
    }
}

impl Mul<f64> for Norm3D {
    type Output = Point3D;

    fn mul(self, other: f64) -> Point3D {
        Point3D {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl Mul<f64> for &Norm3D {
    type Output = Point3D;

    fn mul(self, other: f64) -> Point3D {
        Point3D {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl Neg for Norm3D {
    type Output = Norm3D;

    fn neg(self) -> Norm3D {
        Norm3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Norm3D {
    type Output = Norm3D;

    fn neg(self) -> Norm3D {
        Norm3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl IsND for Norm3D {
    fn n_dimensions() -> usize {
        3
    }

    fn position_nd(&self, dimension: usize) -> Result<f64> {
        match dimension {
            0 => Ok(self.x),
            1 => Ok(self.y),
            2 => Ok(self.z),
            _ => Err(ErrorKind::IncorrectDimension),
        }
    }
}

impl Is3D for Norm3D {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }
}

impl IsNormalized3D for Norm3D {
    fn new<P>(p: P) -> Result<Self>
    where
        P: Is3D,
    {
        let l = p.abs().get();
        if l == 0.0 {
            return Err(ErrorKind::NormalizeVecWithoutLength);
        }

        let f = 1.0 / l;

        Ok(Norm3D {
            x: f * p.x(),
            y: f * p.y(),
            z: f * p.z(),
        })
    }
}

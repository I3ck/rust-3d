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

//! Point3D, a point / position within 3D space

use crate::utils::hash_f64;
use std::{
    cmp::{Eq, Ordering},
    fmt,
    hash::{Hash, Hasher},
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::*;

//------------------------------------------------------------------------------

#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
/// Point3D, a point / position within 3D space
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }
}

impl Eq for Point3D {}

impl Ord for Point3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point3D::default();
        sqr_dist_3d(&origin, self)
            .partial_cmp(&sqr_dist_3d(&origin, other))
            .unwrap_or(Ordering::Equal)
    }
}

impl Hash for Point3D {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_f64(self.x, state);
        hash_f64(self.y, state);
        hash_f64(self.z, state);
    }
}

impl<P> Add<P> for Point3D
where
    P: Is3D,
{
    type Output = Point3D;

    fn add(self, other: P) -> Point3D {
        Point3D {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl<P> Add<&P> for &Point3D
where
    P: Is3D,
{
    type Output = Point3D;

    fn add(self, other: &P) -> Point3D {
        Point3D {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl Add<Point3D> for &Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl<P> Sub<P> for Point3D
where
    P: Is3D,
{
    type Output = Point3D;

    fn sub(self, other: P) -> Point3D {
        Point3D {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl<P> Sub<&P> for &Point3D
where
    P: Is3D,
{
    type Output = Point3D;

    fn sub(self, other: &P) -> Point3D {
        Point3D {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Sub<Point3D> for &Point3D {
    type Output = Point3D;

    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, other: f64) -> Point3D {
        Point3D {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl Mul<f64> for &Point3D {
    type Output = Point3D;

    fn mul(self, other: f64) -> Point3D {
        Point3D {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl Div<f64> for Point3D {
    type Output = Point3D;

    fn div(self, other: f64) -> Point3D {
        Point3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div<f64> for &Point3D {
    type Output = Point3D;

    fn div(self, other: f64) -> Point3D {
        Point3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Point3D {
    type Output = Point3D;

    fn neg(self) -> Point3D {
        Point3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Point3D {
    type Output = Point3D;

    fn neg(self) -> Point3D {
        Point3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl IsMovable3D for Point3D {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl IsND for Point3D {
    fn n_dimensions() -> usize {
        3
    }

    fn position_nd(&self, dimension: usize) -> Option<f64> {
        match dimension {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            _ => None,
        }
    }
}

impl Is3D for Point3D {
    #[inline(always)]
    fn x(&self) -> f64 {
        self.x
    }
    #[inline(always)]
    fn y(&self) -> f64 {
        self.y
    }
    #[inline(always)]
    fn z(&self) -> f64 {
        self.z
    }
}

impl IsBuildableND for Point3D {
    #[inline(always)]
    fn new_nd(coords: &[f64]) -> Result<Self> {
        if coords.len() != 3 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Point3D {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }

    #[inline(always)]
    fn from_nd<P>(&mut self, other: P) -> Result<()>
    where
        P: IsBuildableND,
    {
        if P::n_dimensions() != 3 {
            return Err(ErrorKind::DimensionsDontMatch);
        }

        self.x = other.position_nd(0).ok_or(ErrorKind::IncorrectDimension)?;
        self.y = other.position_nd(1).ok_or(ErrorKind::IncorrectDimension)?;
        self.z = other.position_nd(2).ok_or(ErrorKind::IncorrectDimension)?;
        Ok(())
    }
}

impl IsBuildable3D for Point3D {
    #[inline(always)]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    #[inline(always)]
    fn from<P>(&mut self, other: &P)
    where
        P: Is3D,
    {
        self.x = other.x();
        self.y = other.y();
        self.z = other.z();
    }
}

impl IsEditableND for Point3D {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        match dimension {
            0 => self.x = val,
            1 => self.y = val,
            2 => self.z = val,
            _ => return Err(ErrorKind::DimensionsDontMatch),
        }
        Ok(())
    }
}

impl IsEditable3D for Point3D {
    #[inline(always)]
    fn set_x(&mut self, val: f64) {
        self.x = val;
    }

    #[inline(always)]
    fn set_y(&mut self, val: f64) {
        self.y = val;
    }

    #[inline(always)]
    fn set_z(&mut self, val: f64) {
        self.z = val;
    }
}

impl IsTransFormableTo2D for Point3D {
    fn transform_to_2d<P>(&self) -> P
    where
        P: IsBuildable2D,
    {
        P::new(self.x, self.y)
    }
}

impl IsMatrix4Transformable for Point3D {
    fn transformed(&self, m: &Matrix4) -> Self {
        self.multiply_m(m)
    }
    fn transform(&mut self, m: &Matrix4) {
        let new = self.multiply_m(m);
        *self = new;
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

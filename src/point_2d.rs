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

//! Point2D, a point / position within 2D space

use std::{
    cmp::{Eq, Ordering},
    fmt,
    hash::{Hash, Hasher},
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::*;

//------------------------------------------------------------------------------

#[derive(Default, Debug, PartialEq, PartialOrd, Clone)]
/// Point2D, a point / position within 2D space
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
}

impl Eq for Point2D {}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point2D::default();
        sqr_dist_2d(&origin, self)
            .partial_cmp(&sqr_dist_2d(&origin, other))
            .unwrap_or(Ordering::Equal)
    }
}

impl Hash for Point2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_f64(self.x, state);
        hash_f64(self.y, state);
    }
}

impl<P> Add<P> for Point2D
where
    P: Is2D,
{
    type Output = Point2D;

    fn add(self, other: P) -> Point2D {
        Point2D {
            x: self.x + other.x(),
            y: self.y + other.y(),
        }
    }
}

impl<P> Add<&P> for &Point2D
where
    P: Is2D,
{
    type Output = Point2D;

    fn add(self, other: &P) -> Point2D {
        Point2D {
            x: self.x + other.x(),
            y: self.y + other.y(),
        }
    }
}

impl Add<Point2D> for &Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x(),
            y: self.y + other.y(),
        }
    }
}

impl<P> Sub<P> for Point2D
where
    P: Is2D,
{
    type Output = Point2D;

    fn sub(self, other: P) -> Point2D {
        Point2D {
            x: self.x - other.x(),
            y: self.y - other.y(),
        }
    }
}

impl<P> Sub<&P> for &Point2D
where
    P: Is2D,
{
    type Output = Point2D;

    fn sub(self, other: &P) -> Point2D {
        Point2D {
            x: self.x - other.x(),
            y: self.y - other.y(),
        }
    }
}

impl Sub<Point2D> for &Point2D {
    type Output = Point2D;

    fn sub(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x - other.x(),
            y: self.y - other.y(),
        }
    }
}

impl Mul<f64> for Point2D {
    type Output = Point2D;

    fn mul(self, other: f64) -> Point2D {
        Point2D {
            x: other * self.x,
            y: other * self.y,
        }
    }
}

impl Mul<f64> for &Point2D {
    type Output = Point2D;

    fn mul(self, other: f64) -> Point2D {
        Point2D {
            x: other * self.x,
            y: other * self.y,
        }
    }
}

impl Div<f64> for Point2D {
    type Output = Point2D;

    fn div(self, other: f64) -> Point2D {
        Point2D {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Div<f64> for &Point2D {
    type Output = Point2D;

    fn div(self, other: f64) -> Point2D {
        Point2D {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Neg for Point2D {
    type Output = Point2D;

    fn neg(self) -> Point2D {
        Point2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Neg for &Point2D {
    type Output = Point2D;

    fn neg(self) -> Point2D {
        Point2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl IsMovable2D for Point2D {
    fn move_by(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
}

impl IsND for Point2D {
    fn n_dimensions() -> usize {
        2
    }

    fn position_nd(&self, dimension: usize) -> Result<f64> {
        match dimension {
            0 => Ok(self.x),
            1 => Ok(self.y),
            _ => Err(ErrorKind::IncorrectDimension),
        }
    }
}

impl Is2D for Point2D {
    #[inline(always)]
    fn x(&self) -> f64 {
        self.x
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        self.y
    }
}

impl IsBuildableND for Point2D {
    #[inline(always)]
    fn new_nd(coords: &[f64]) -> Result<Self> {
        if coords.len() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Point2D {
            x: coords[0],
            y: coords[1],
        })
    }

    #[inline(always)]
    fn from_nd<P>(&mut self, other: P) -> Result<()>
    where
        P: IsBuildableND,
    {
        if P::n_dimensions() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }

        self.x = other.position_nd(0)?;
        self.y = other.position_nd(1)?;
        Ok(())
    }
}

impl IsBuildable2D for Point2D {
    #[inline(always)]
    fn new(x: f64, y: f64) -> Self {
        Point2D { x: x, y: y }
    }

    #[inline(always)]
    fn from<P>(&mut self, other: &P)
    where
        P: Is2D,
    {
        self.x = other.x();
        self.y = other.y();
    }
}

impl IsEditableND for Point2D {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        match dimension {
            0 => self.x = val,
            1 => self.y = val,
            _ => return Err(ErrorKind::DimensionsDontMatch),
        }
        Ok(())
    }
}

impl IsEditable2D for Point2D {
    #[inline(always)]
    fn set_x(&mut self, val: f64) {
        self.x = val;
    }

    #[inline(always)]
    fn set_y(&mut self, val: f64) {
        self.y = val;
    }
}

impl IsTransFormableTo3D for Point2D {
    fn transform_to_3d<P>(&self, z: f64) -> P
    where
        P: IsBuildable3D,
    {
        P::new(self.x, self.y, z)
    }
}

impl IsMatrix3Transformable for Point2D {
    fn transformed(&self, m: &Matrix3) -> Self {
        self.multiply_m(m)
    }
    fn transform(&mut self, m: &Matrix3) {
        let new = self.multiply_m(m);
        *self = new;
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

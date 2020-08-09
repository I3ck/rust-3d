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

//! Norm2D, a normalized vector within 2D space

use std::{
    cmp::{Eq, Ordering},
    hash::{Hash, Hasher},
    ops::{Mul, Neg},
};

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// Norm2D, a normalized vector within 2D space
pub struct Norm2D {
    x: f64,
    y: f64,
}

impl Eq for Norm2D {}

impl Ord for Norm2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point2D::default();
        sqr_dist_2d(&origin, self)
            .partial_cmp(&sqr_dist_2d(&origin, other))
            .unwrap_or(Ordering::Equal)
    }
}

impl Hash for Norm2D {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_f64(self.x, state);
        hash_f64(self.y, state);
    }
}

impl Mul<f64> for Norm2D {
    type Output = Point2D;

    fn mul(self, other: f64) -> Point2D {
        Point2D {
            x: other * self.x,
            y: other * self.y,
        }
    }
}

impl Mul<f64> for &Norm2D {
    type Output = Point2D;

    fn mul(self, other: f64) -> Point2D {
        Point2D {
            x: other * self.x,
            y: other * self.y,
        }
    }
}

impl Neg for Norm2D {
    type Output = Norm2D;

    fn neg(self) -> Norm2D {
        Norm2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Neg for &Norm2D {
    type Output = Norm2D;

    fn neg(self) -> Norm2D {
        Norm2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl IsND for Norm2D {
    fn n_dimensions() -> usize {
        2
    }

    fn position_nd(&self, dimension: usize) -> Option<f64> {
        match dimension {
            0 => Some(self.x),
            1 => Some(self.y),
            _ => None,
        }
    }
}

impl Is2D for Norm2D {
    #[inline(always)]
    fn x(&self) -> f64 {
        self.x
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        self.y
    }
}

impl IsNormalized2D for Norm2D {
    fn new<P>(p: P) -> Result<Self>
    where
        P: Is2D,
    {
        let l = p.abs().get();
        if l == 0.0 {
            return Err(ErrorKind::NormalizeVecWithoutLength);
        }

        let f = 1.0 / l;

        Ok(Norm2D {
            x: f * p.x(),
            y: f * p.y(),
        })
    }
}

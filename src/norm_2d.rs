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

//! Norm2D, a normalized vector within 2D space

use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};
use std::ops::{Mul, Neg};

use prelude::*;
use distances_2d::*;

#[derive (Debug, PartialEq, PartialOrd, Clone)]
/// Norm2D, a normalized vector within 2D space
pub struct Norm2D {
    x: f64,
    y: f64
}

impl Eq for Norm2D {}

impl Ord for Norm2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point2D::default();
        sqr_dist_2d(&origin, self).partial_cmp(&sqr_dist_2d(&origin, other)).unwrap_or(Ordering::Equal)
    }
}

impl Hash for Norm2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
    }
}

impl Mul<f64> for Norm2D {
    type Output = Point2D;

    fn mul(self, other: f64) -> Point2D {
        Point2D{x: other * self.x, y: other * self.y}
    }
}

impl Neg for Norm2D {
    type Output = Norm2D;

    fn neg(self) -> Norm2D {
        Norm2D { x: -self.x, y: -self.y }
    }
}

impl IsND for Norm2D {
    fn n_dimensions() -> usize {
        2
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        match dimension {
            0 => Ok(self.x),
            1 => Ok(self.y),
            _ => Err(ErrorKind::IncorrectDimension)
        }
    }
}

impl Is2D for Norm2D {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl IsNormalized2D for Norm2D {
    fn new<P>(p: P) -> Result<Self> where
        P: Is2D {

        let l = p.abs();
        if l.get() == 0.0 {
            return Err(ErrorKind::NormalizeVecWithoutLength);
        }

        Ok(Norm2D { x: p.x() / l.get(), y: p.y() / l.get() })
    }

    fn norm_x() -> Self {
        Norm2D {
            x: 1.0,
            y: 0.0
        }
    }

    fn norm_y() -> Self {
        Norm2D {
            x: 0.0,
            y: 1.0
        }
    }
}

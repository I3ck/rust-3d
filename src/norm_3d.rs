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

//! Norm3D, a normalized vector within 3D space

use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};
use std::ops::{Mul};

use result::*;
use point_3d::*;
use traits::is_nd::*;
use traits::is_3d::*;
use traits::is_normalized_3d::*;
use traits::is_buildable_3d::*;
use distances_3d::*;

#[derive (PartialEq, PartialOrd, Clone)]
/// Norm3D, a normalized vector within 3D space
pub struct Norm3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Eq for Norm3D{}

impl Ord for Norm3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point3D::default();
        sqr_dist_3d(&origin, self).partial_cmp(&sqr_dist_3d(&origin, other)).unwrap_or(Ordering::Equal)
    }
}

impl Hash for Norm3D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
        (self.z as u64).hash(state);
    }
}

impl Mul<f64> for Norm3D {
    type Output = Point3D; //@todo could later be another type

    fn mul(self, other: f64) -> Point3D {
        *Point3D::new(other * self.x, other * self.y, other * self.z)
    }
}

impl IsND for Norm3D {
    fn n_dimensions() -> usize {
        3
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        match dimension {
            0 => Ok(self.x),
            1 => Ok(self.y),
            2 => Ok(self.z),
            _ => Err(ErrorKind::IncorrectDimension)
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
    fn new<P>(p: P) -> Result<Box<Self>> where
        P: Is3D {

        match p.abs() {
            0.0 => Err(ErrorKind::NormalizeVecWithoutLength),
            l => Ok(Box::new(Norm3D {
                x: p.x() / l,
                y: p.y() / l,
                z: p.z() / l,
            }))
        }
    }

    fn norm_x() -> Self {
        Norm3D {
            x: 1.0,
            y: 0.0,
            z: 0.0
        }
    }

    fn norm_y() -> Self {
        Norm3D {
            x: 0.0,
            y: 1.0,
            z: 0.0
        }
    }

    fn norm_z() -> Self {
        Norm3D {
            x: 0.0,
            y: 0.0,
            z: 1.0
        }
    }
}

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

use std::fmt;
use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};

use result::*;
use traits::is_nd::IsND;
use traits::is_3d::Is3D;
use traits::is_moveable_3d::IsMoveable3D;
use traits::is_buildable_2d::IsBuildable2D;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_editable_3d::IsEditable3D;
use traits::transformable_to_2d::TransFormableTo2D;
use functions::{sqr_dist_3d};

#[derive (PartialEq, PartialOrd)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Eq for Point3D {}

impl Ord for Point3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point3D::new();
        sqr_dist_3d(&origin, self).partial_cmp(&sqr_dist_3d(&origin, other)).unwrap_or(Ordering::Equal)
    }
}

impl Hash for Point3D { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
        (self.z as u64).hash(state);
    }
}

impl Clone for Point3D {
    fn clone(&self) -> Point3D {
        Point3D { x: self.x, y: self.y, z: self.z }
    }
}

impl IsMoveable3D for Point3D {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl IsND for Point3D {
    fn n_dimensions(&self) -> usize {
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

impl Is3D for Point3D {
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

impl IsBuildable3D for Point3D {
    fn new() -> Box<Self> {
        Box::new(Point3D{x: 0.0, y: 0.0, z: 0.0})
    }

    fn build(x: f64, y: f64, z: f64) -> Box<Self> {
        Box::new(Point3D{x: x, y: y, z: z})
    }

    fn from<P>(&mut self, other: P) where P: IsBuildable3D {
        self.x = other.x();
        self.y = other.y();
        self.z = other.z();
    }
}

impl IsEditable3D for Point3D {
    fn set_x(&mut self, val: f64) {
        self.x = val;
    }

    fn set_y(&mut self, val: f64) {
        self.y = val;
    }

    fn set_z(&mut self, val: f64) {
        self.z = val;
    }
}

impl TransFormableTo2D for Point3D {
    fn transform_to_2d<P>(&self) -> P where
        P: IsBuildable2D {

        *P::build(self.x, self.y)
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

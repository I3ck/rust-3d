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

//! Point2D, a point / position within 2D space

use std::fmt;
use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};


use result::*;
use traits::is_nd::*;
use traits::is_2d::*;
use traits::is_moveable_2d::*;
use traits::is_buildable_nd::*;
use traits::is_buildable_2d::*;
use traits::is_editable_nd::*;
use traits::is_editable_2d::*;
use traits::is_buildable_3d::*;
use traits::is_transformable_to_3d::*;
use functions::{sqr_dist_2d};

#[derive (PartialEq, PartialOrd, Default)]
/// Point2D, a point / position within 2D space
pub struct Point2D {
    pub x: f64,
    pub y: f64
}

impl Eq for Point2D {}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point2D::default();
        sqr_dist_2d(&origin, self).partial_cmp(&sqr_dist_2d(&origin, other)).unwrap_or(Ordering::Equal)
    }
}

impl Hash for Point2D { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
    }
}

impl Clone for Point2D {
    fn clone(&self) -> Self {
        Point2D { x: self.x, y: self.y }
    }
}

impl IsMoveable2D for Point2D {
    fn move_by(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
}

impl IsND for Point2D {
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

impl Is2D for Point2D {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl IsBuildableND for Point2D {
    fn build_nd(coords: &Vec<f64>) -> Result<Box<Self>> {
        if coords.len() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Box::new(Point2D{x: coords[0], y: coords[1]}))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        if P::n_dimensions() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }

        self.x = other.get_position(0)?;
        self.y = other.get_position(1)?;
        Ok(())
    }
}

impl IsBuildable2D for Point2D {
    fn build(x: f64, y: f64) -> Box<Self> {
        Box::new(Point2D{x: x, y: y})
    }

    fn from<P>(&mut self, other: P) where P: IsBuildable2D {
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
    fn set_x(&mut self, val: f64) {
        self.x = val;
    }

    fn set_y(&mut self, val: f64) {
        self.y = val;
    }
}

impl IsTransFormableTo3D for Point2D {
    fn transform_to_3d<P>(&self, z: f64) -> P where
        P: IsBuildable3D {

        *P::build(self.x, self.y, z)
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

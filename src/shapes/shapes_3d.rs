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

//! Containing basic 3D shape data types used by other types / algorithms

use std::cmp::{Eq, Ordering};

use prelude::*;
use distances_3d::*;

#[derive (Debug, PartialEq, PartialOrd, Clone, Hash, Default)]
/// Sphere, a sphere in 3D space
pub struct Sphere {
    pub center: Point2D,
    pub radius: Positive
}
impl Eq for Sphere {}












#[derive (Debug, PartialEq, PartialOrd, Clone, Hash, Default)]
/// Box3D, a box in 3D space
pub struct Box3D {
    pub center: Point3D,
    pub size_x: Positive,
    pub size_y: Positive,
    pub size_z: Positive
}
impl Eq for Box3D {}


impl Ord for Box3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point3D::default();
        match sqr_dist_3d(&origin, &self.center).partial_cmp(&sqr_dist_3d(&origin, &other.center)) {
            Some(x) => x,
            None => match self.size_x.partial_cmp(&other.size_x) {
                Some(x) => x,
                None => match self.size_y.partial_cmp(&other.size_y) {
                    Some(x) => x,
                    None => self.size_z.partial_cmp(&other.size_z).unwrap_or(Ordering::Equal)
                }
            }
        }
    }
}

impl IsND for Box3D {
    fn n_dimensions() -> usize {
        3
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.center.get_position(dimension)
    }
}

impl Is3D for Box3D {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }

    fn z(&self) -> f64 {
        self.center.z()
    }
}

impl IsBuildableND for Box3D {
    fn new_nd(coords: &Vec<f64>) -> Result<Box<Self>> {
        if coords.len() != 3 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Box::new(Box3D{center: Point3D{x: coords[0], y: coords[1], z: coords[2]}, size_x: Positive::one(), size_y: Positive::one(), size_z: Positive::one()}))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        if P::n_dimensions() != 3 {
            return Err(ErrorKind::DimensionsDontMatch);
        }

        self.center.set_x(other.get_position(0)?);
        self.center.set_y(other.get_position(1)?);
        self.center.set_z(other.get_position(2)?);
        Ok(())
    }
}

impl IsBuildable3D for Box3D {
    fn new(x: f64, y: f64, z: f64) -> Box<Self> {
        Box::new(Box3D{center: Point3D{x: x, y: y, z: z}, size_x: Positive::one(), size_y: Positive::one(), size_z: Positive::one()})
    }

    fn from<P>(&mut self, other: P)
        where P: Is3D {

        self.center.from(other)
    }
}

impl IsEditableND for Box3D {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        match dimension {
            0 => self.center.set_x(val),
            1 => self.center.set_y(val),
            2 => self.center.set_z(val),
            _ => return Err(ErrorKind::DimensionsDontMatch),
        }
        Ok(())
    }
}

impl IsEditable3D for Box3D {
    fn set_x(&mut self, val: f64) {
        self.center.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.center.set_y(val);
    }

    fn set_z(&mut self, val: f64) {
        self.center.set_z(val);
    }
}

impl HasBoundingBox3D for Box3D {
    fn bounding_box(&self) -> Result<BoundingBox3D> {
        let p_min = Point3D{x: self.center.x() - self.size_x.get() / 2.0, y: self.center.y() - self.size_y.get() / 2.0, z: self.center.z() - self.size_z.get() / 2.0};
        let p_max = Point3D{x: self.center.x() + self.size_x.get() / 2.0, y: self.center.y() + self.size_y.get() / 2.0, z: self.center.z() + self.size_z.get() / 2.0};
        BoundingBox3D::new(&p_min, &p_max)
    }
}

impl From<BoundingBox3D> for Box3D {
    fn from(x: BoundingBox3D) -> Self {
        Box3D{center: x.center_bb(), size_x: x.size_x(), size_y: x.size_y(), size_z: x.size_z()}
    }
}

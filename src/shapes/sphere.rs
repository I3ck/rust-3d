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

//! Sphere, a sphere in 3D space

use std::cmp::{Eq, Ordering};

use prelude::*;
use distances_3d::*;

#[derive (Debug, PartialEq, PartialOrd, Clone, Hash, Default)]
/// Sphere, a sphere in 3D space
pub struct Sphere {
    pub center: Point3D,
    pub radius: Positive
}

impl Eq for Sphere {}

impl Ord for Sphere {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point3D::default();
        match sqr_dist_3d(&origin, &self.center).partial_cmp(&sqr_dist_3d(&origin, &other.center)) {
            Some(x) => x,
            None => self.radius.partial_cmp(&other.radius).unwrap_or(Ordering::Equal)
        }
    }
}

impl IsND for Sphere {
    fn n_dimensions() -> usize {
        Point3D::n_dimensions()
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.center.get_position(dimension)
    }
}

impl Is3D for Sphere {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }

    fn z(&self) -> f64 {
        self.center.y()
    }
}

impl IsBuildableND for Sphere {
    fn new_nd(coords: &[f64]) -> Result<Self> {
        Ok(Sphere{center: Point3D::new_nd(coords)?, radius: Positive::one()})
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        self.center.from_nd(other)
    }
}

impl IsBuildable3D for Sphere {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Sphere{center: Point3D{x, y, z}, radius: Positive::one()}
    }

    fn from<P>(&mut self, other: P)
        where P: Is3D {

        self.center.from(other)
    }
}

impl IsEditableND for Sphere {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        self.center.set_position(dimension, val)
    }
}

impl IsEditable3D for Sphere {
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

impl HasBoundingBox3D for Sphere {
    fn bounding_box(&self) -> Result<BoundingBox3D> {
        let p_min = Point3D{x: self.center.x() - self.radius.get(), y: self.center.y() - self.radius.get(), z: self.center.z() - self.radius.get()};
        let p_max = Point3D{x: self.center.x() + self.radius.get(), y: self.center.y() + self.radius.get(), z: self.center.z() + self.radius.get()};
        BoundingBox3D::new(&p_min, &p_max)
    }
}

impl IsScalable for Sphere {
    fn scale(&mut self, factor: Positive) {
        self.radius *= factor;
    }
}

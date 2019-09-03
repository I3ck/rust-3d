/*
Copyright 2017 Martin Buck

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

    fn position_nd(&self, dimension: usize) -> Result<f64> {
        self.center.position_nd(dimension)
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

    fn from<P>(&mut self, other: &P)
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
    fn bounding_box(&self) -> BoundingBox3D {
        let p_min = Point3D{x: self.center.x() - self.radius.get(), y: self.center.y() - self.radius.get(), z: self.center.z() - self.radius.get()};
        let p_max = Point3D{x: self.center.x() + self.radius.get(), y: self.center.y() + self.radius.get(), z: self.center.z() + self.radius.get()};
        BoundingBox3D::new(&p_min, &p_max).unwrap() // safe
    }
}

impl HasBoundingBox3DMaybe for Sphere {
    fn bounding_box_maybe(&self) -> Result<BoundingBox3D> {
        Ok(self.bounding_box())
    }
}

impl IsScalable for Sphere {
    fn scale(&mut self, factor: Positive) {
        self.radius *= factor;
    }
}

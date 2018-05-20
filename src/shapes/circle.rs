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

//! Circle, a circle in 2D space

use std::cmp::{Eq, Ordering};

use prelude::*;
use distances_2d::*;

#[derive (Debug, PartialEq, PartialOrd, Clone, Hash, Default)]
/// Circle, a circle in 2D space
pub struct Circle {
    pub center: Point2D,
    pub radius: Positive
}

impl Eq for Circle {}

impl Ord for Circle {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point2D::default();
        match sqr_dist_2d(&origin, &self.center).partial_cmp(&sqr_dist_2d(&origin, &other.center)) {
            Some(x) => x,
            None => self.radius.partial_cmp(&other.radius).unwrap_or(Ordering::Equal)
        }
    }
}

impl IsND for Circle {
    fn n_dimensions() -> usize {
        Point2D::n_dimensions()
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.center.get_position(dimension)
    }
}

impl Is2D for Circle {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }
}

impl IsBuildableND for Circle {
    fn new_nd(coords: &[f64]) -> Result<Self> {
        Ok(Circle{ center: Point2D::new_nd(coords)?, radius: Positive::one()})
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        self.center.from_nd(other)
    }
}

impl IsBuildable2D for Circle {
    fn new(x: f64, y: f64) -> Self {
        Circle{ center: Point2D{x: x, y: y}, radius: Positive::one()}
    }

    fn from<P>(&mut self, other: P)
        where P: Is2D {

        self.center.from(other)
    }
}

impl IsEditableND for Circle {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        self.center.set_position(dimension, val)
    }
}

impl IsEditable2D for Circle {
    fn set_x(&mut self, val: f64) {
        self.center.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.center.set_y(val);
    }
}

impl HasBoundingBox2D for Circle {
    fn bounding_box(&self) -> Result<BoundingBox2D> {
        let p_min = Point2D{x: self.center.x() - self.radius.get(), y: self.center.y() - self.radius.get()};
        let p_max = Point2D{x: self.center.x() + self.radius.get(), y: self.center.y() + self.radius.get()};
        BoundingBox2D::new(&p_min, &p_max)
    }
}

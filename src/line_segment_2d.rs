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

//! LineSegment2D, a line segment within 2D space

use std::fmt;

use prelude::*;
use distances_2d::dist_2d;
use functions::center_2d;

#[derive (Debug, PartialEq, PartialOrd, Eq, Clone, Hash)]
/// LineSegment2D, a line segment within 2D space
pub struct LineSegment2D {
    pub start: Point2D,
    pub end: Point2D
}

impl LineSegment2D {
    /// Creates a new LineSegment2D from a start and end Point
    pub fn new(start: Point2D, end: Point2D) -> Self {
        LineSegment2D{start, end}
    }
}

impl IsMovable2D for LineSegment2D {
    fn move_by(&mut self, x: f64, y: f64) {
        self.start.move_by(x, y);
        self.end.move_by(x, y);
    }
}

impl HasLength for LineSegment2D {
    fn length(&self) -> f64 {
       dist_2d(&self.start, &self.end)
    }
}

impl HasBoundingBox2D for LineSegment2D {
    fn bounding_box(&self) -> Result<BoundingBox2D> {
        BoundingBox2D::from_iterator(&[self.start.clone(), self.end.clone()])
    }
}

impl HasCenterOfGravity2D for LineSegment2D {
    fn center_of_gravity(&self) -> Result<Point2D> {
        Ok(center_2d(&self.start, &self.end))
    }
}

impl IsScalable for LineSegment2D {
    fn scale(&mut self, factor: Positive) {
        let c = self.bounding_box().unwrap().center_bb(); //always known

        self.start.increase_distance_to_by(&c, factor);
        self.end.increase_distance_to_by(&c, factor);
    }
}

impl IsMatrix3Transformable for LineSegment2D {
    fn transformed(&self, m: &Matrix3) -> Self {
        let mut new = self.clone();
        new.transform(m);
        new
    }

    fn transform(&mut self, m: &Matrix3) {
        self.start.transform(m);
        self.end.transform(m);
    }
}

impl fmt::Display for LineSegment2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {} -> {}, {})", self.start.x(), self.start.y(), self.end.x(), self.end.y())
    }
}

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

//! LineSegment3D, a line segment within 3D space

use std::fmt;

use prelude::*;
use distances_3d::dist_3d;
use functions::center_3d;

#[derive (Debug, PartialEq, PartialOrd, Eq, Clone, Hash)]
/// LineSegment3D, a line segment within 3D space
pub struct LineSegment3D {
    pub start: Point3D,
    pub end: Point3D
}

impl LineSegment3D {
    /// Creates a new LineSegment3D from a start and end point
    pub fn new(start: Point3D, end: Point3D) -> Self {
        LineSegment3D{start, end}
    }
}

impl IsMovable3D for LineSegment3D {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.start.move_by(x, y, z);
        self.end.move_by(x, y, z);
    }
}

impl HasLength for LineSegment3D {
    fn length(&self) -> f64 {
        dist_3d(&self.start, &self.end)
    }
}

impl HasBoundingBox3D for LineSegment3D {
    fn bounding_box(&self) -> Result<BoundingBox3D> {
        let mut pts = Vec::new();
        pts.push(Box::new(self.start.clone()));
        pts.push(Box::new(self.end.clone()));
        BoundingBox3D::from_iterator(pts.iter())
    }
}

impl HasCenterOfGravity3D for LineSegment3D {
    fn center_of_gravity(&self) -> Result<Point3D> {
        Ok(center_3d(&self.start, &self.end))
    }
}

impl IsScalable for LineSegment3D {
    fn scale(&mut self, factor: Positive) {
        let c = self.bounding_box().unwrap().center_bb(); //always known

        self.start.increase_distance_to_by(&c, factor);
        self.end.increase_distance_to_by(&c, factor);
    }
}

impl fmt::Display for LineSegment3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {} -> {}, {}, {})", self.start.x(), self.start.y(), self.start.z(), self.end.x(), self.end.y(), self.end.z())
    }
}

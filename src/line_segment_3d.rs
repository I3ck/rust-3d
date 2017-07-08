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

use result::*;
use traits::is_3d::*;
use traits::has_length::*;
use traits::has_bounding_box_3d::*;
use traits::has_center_of_gravity_3d::*;
use traits::is_movable_3d::*;
use point_3d::*;
use bounding_box_3d::*;

use line_3d::*;
use positive::*;

#[derive (PartialEq, PartialOrd, Eq, Clone, Hash)]
/// LineSegment3D, a line segment within 3D space
pub struct LineSegment3D {
    pub line: Line3D,
    pub length: Positive,
}

impl IsMovable3D for LineSegment3D {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.line.move_by(x, y, z);
    }
}

impl HasLength for LineSegment3D {
    fn length(&self) -> f64 {
        self.length.get()
    }
}

impl HasBoundingBox3D for LineSegment3D {
    fn bounding_box(&self) -> Result<BoundingBox3D> {
        let mut pts = Vec::new();
        pts.push(Box::new(self.line.source.clone()));
        pts.push(Box::new(self.line.source.clone() + self.line.dir.clone() * self.length.get()));
        BoundingBox3D::from_iterator(pts.iter())
    }
}

impl HasCenterOfGravity3D for LineSegment3D {
    fn center_of_gravity(&self) -> Result<Point3D> {
        Ok(self.line.source.clone() + self.line.dir.clone() * 0.5 * self.length.get())
    }
}

impl fmt::Display for LineSegment3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {} -> {} * {}, {}, {})", self.line.source.x(), self.line.source.y(), self.line.source.z(), self.length, self.line.dir.x(), self.line.dir.y(), self.line.dir.z())
    }
}

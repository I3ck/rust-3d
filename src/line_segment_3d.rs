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

//! LineSegment3D, a line segment within 3D space

use std::fmt;

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Hash)]
/// LineSegment3D, a line segment within 3D space
pub struct LineSegment3D {
    pub start: Point3D,
    pub end: Point3D,
}

impl LineSegment3D {
    /// Creates a new LineSegment3D from a start and end point
    pub fn new(start: Point3D, end: Point3D) -> Self {
        LineSegment3D { start, end }
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

impl HasBoundingBox3DMaybe for LineSegment3D {
    fn bounding_box_maybe(&self) -> Option<BoundingBox3D> {
        BoundingBox3D::from_iterator([&self.start, &self.end].iter().map(|x| *x)).ok()
    }
}

impl HasCenterOfGravity3D for LineSegment3D {
    fn center_of_gravity(&self) -> Result<Point3D> {
        Ok(center_3d(&self.start, &self.end))
    }
}

impl IsScalable for LineSegment3D {
    fn scale(&mut self, factor: Positive) {
        if let Some(c) = self.bounding_box_maybe().map(|x| x.center_bb()) {
            self.start.increase_distance_to_by(&c, factor);
            self.end.increase_distance_to_by(&c, factor);
        }
    }
}

impl IsMatrix4Transformable for LineSegment3D {
    fn transformed(&self, m: &Matrix4) -> Self {
        let mut new = self.clone();
        new.transform(m);
        new
    }

    fn transform(&mut self, m: &Matrix4) {
        self.start.transform(m);
        self.end.transform(m);
    }
}

impl fmt::Display for LineSegment3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {} -> {}, {}, {})",
            self.start.x(),
            self.start.y(),
            self.start.z(),
            self.end.x(),
            self.end.y(),
            self.end.z()
        )
    }
}

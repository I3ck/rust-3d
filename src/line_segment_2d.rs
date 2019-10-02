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

//! LineSegment2D, a line segment within 2D space

use std::fmt;

use crate::distances_2d::dist_2d;
use crate::functions::center_2d;
use crate::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Hash)]
/// LineSegment2D, a line segment within 2D space
pub struct LineSegment2D {
    pub start: Point2D,
    pub end: Point2D,
}

impl LineSegment2D {
    /// Creates a new LineSegment2D from a start and end Point
    pub fn new(start: Point2D, end: Point2D) -> Self {
        LineSegment2D { start, end }
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

impl HasBoundingBox2DMaybe for LineSegment2D {
    fn bounding_box_maybe(&self) -> Result<BoundingBox2D> {
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
        if let Ok(c) = self.bounding_box_maybe().map(|x| x.center_bb()) {
            self.start.increase_distance_to_by(&c, factor);
            self.end.increase_distance_to_by(&c, factor);
        }
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
        write!(
            f,
            "({}, {} -> {}, {})",
            self.start.x(),
            self.start.y(),
            self.end.x(),
            self.end.y()
        )
    }
}

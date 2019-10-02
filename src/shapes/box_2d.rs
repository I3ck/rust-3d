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

//! Box2D, a box in 2D space

use std::cmp::{Eq, Ordering};

use crate::prelude::*;
use crate::distances_2d::*;

#[derive (Debug, PartialEq, PartialOrd, Clone, Hash, Default)]
/// Box2D, a box in 2D space
pub struct Box2D {
    pub center: Point2D,
    pub size_x: Positive,
    pub size_y: Positive
}

impl Box2D {
    /// Returns the minimum position of the box
    pub fn min_p(&self) -> Point2D {
        Point2D::new(self.center.x() - 0.5*self.size_x.get(), self.center.y() - 0.5*self.size_y.get())
    }
    /// Returns the maximum position of the box
    pub fn max_p(&self) -> Point2D {
        Point2D::new(self.center.x() + 0.5*self.size_x.get(), self.center.y() + 0.5*self.size_y.get())
    }
    /// Returns the sizes of the bounding box
    pub fn sizes(&self) -> (Positive, Positive) {
        (self.size_x, self.size_y)
    }
}

impl Eq for Box2D {}

impl Ord for Box2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point2D::default();
        match sqr_dist_2d(&origin, &self.center).partial_cmp(&sqr_dist_2d(&origin, &other.center)) {
            Some(x) => x,
            None => match self.size_x.partial_cmp(&other.size_x) {
                Some(x) => x,
                None => self.size_y.partial_cmp(&other.size_y).unwrap_or(Ordering::Equal)
            }
        }
    }
}

impl IsND for Box2D {
    fn n_dimensions() -> usize {
        Point2D::n_dimensions()
    }

    fn position_nd(&self, dimension: usize) -> Result<f64> {
        self.center.position_nd(dimension)
    }
}

impl Is2D for Box2D {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }
}

impl IsBuildableND for Box2D {
    fn new_nd(coords: &[f64]) -> Result<Self> {
        Ok(Box2D{center: Point2D::new_nd(coords)?,  size_x: Positive::one(), size_y: Positive::one()})
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        self.center.from_nd(other)
    }
}

impl IsBuildable2D for Box2D {
    fn new(x: f64, y: f64) -> Self {
        Box2D{center: Point2D{x: x, y: y}, size_x: Positive::one(), size_y: Positive::one()}
    }

    fn from<P>(&mut self, other: &P) where
        P: Is2D {

        self.center.from(other)
    }
}

impl IsEditableND for Box2D {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        self.center.set_position(dimension, val)
    }
}

impl IsEditable2D for Box2D {
    fn set_x(&mut self, val: f64) {
        self.center.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.center.set_y(val);
    }
}

impl HasBoundingBox2D for Box2D {
    fn bounding_box(&self) -> BoundingBox2D {
        let p_min = Point2D{x: self.center.x() - self.size_x.get() / 2.0, y: self.center.y() - self.size_y.get() / 2.0};
        let p_max = Point2D{x: self.center.x() + self.size_x.get() / 2.0, y: self.center.y() + self.size_y.get() / 2.0};
        BoundingBox2D::new(&p_min, &p_max).unwrap() // safe
    }
}

impl HasBoundingBox2DMaybe for Box2D {
    fn bounding_box_maybe(&self) -> Result<BoundingBox2D> {
        Ok(self.bounding_box())
    }
}

impl IsScalable for Box2D {
    fn scale(&mut self, factor: Positive) {
        self.size_x *= factor;
        self.size_y *= factor;
    }
}

impl IsMovable2D for Box2D {
    fn move_by(&mut self, x: f64, y: f64) {
        self.center.move_by(x, y)
    }
}

impl From<BoundingBox2D> for Box2D {
    fn from(x: BoundingBox2D) -> Self {
        Box2D{center: x.center_bb(), size_x: x.size_x(), size_y: x.size_y()}
    }
}

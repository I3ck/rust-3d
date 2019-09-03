/*
Copyright 2016 Martin Buck

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

//! FilterBox2D, a box filter within 2D space

use prelude::*;

#[derive (Debug, PartialEq, PartialOrd, Default, Clone, Hash, Eq, Ord)]
/// FilterBox2D, a box filter within 2D space
pub struct FilterBox2D {
    box_2d: Box2D
}

impl FilterBox2D {
    /// Creates a new FilterBox2D with the given parameters
    pub fn new(box_2d: Box2D) -> Self {
        FilterBox2D {box_2d}
    }
}

impl IsND for FilterBox2D {
    fn n_dimensions() -> usize {
        Box2D::n_dimensions()
    }

    fn position_nd(&self, dimension: usize) -> Result<f64> {
        self.box_2d.position_nd(dimension)
    }
}

impl Is2D for FilterBox2D {
    fn x(&self) -> f64 {
        self.box_2d.x()
    }

    fn y(&self) -> f64 {
        self.box_2d.y()
    }
}

impl IsBuildableND for FilterBox2D {
    fn new_nd(coords: &[f64]) -> Result<Self> {
        Ok(FilterBox2D::new(Box2D::new_nd(coords)?))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        self.box_2d.from_nd(other)
    }
}

impl IsBuildable2D for FilterBox2D {
    fn new(x: f64, y: f64) -> Self {
        FilterBox2D::new(Box2D::new(x, y))
    }

    fn from<P>(&mut self, other: &P) where
        P: Is2D {

        self.box_2d.from(other)
    }
}

impl IsEditableND for FilterBox2D {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        self.box_2d.set_position(dimension, val)
    }
}

impl IsEditable2D for FilterBox2D {
    fn set_x(&mut self, val: f64) {
        self.box_2d.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.box_2d.set_y(val);
    }
}

impl HasBoundingBox2D for FilterBox2D {
    fn bounding_box(&self) -> BoundingBox2D {
        self.box_2d.bounding_box()
    }
}

impl HasBoundingBox2DMaybe for FilterBox2D {
    fn bounding_box_maybe(&self) -> Result<BoundingBox2D> {
        self.box_2d.bounding_box_maybe()
    }
}

impl<T> IsFilter<T> for FilterBox2D where
    T: Is2D {

    fn is_allowed(&self, p: &T) -> bool {
           p.x() >= self.box_2d.x() - self.box_2d.size_x.get() / 2.0
        && p.x() <= self.box_2d.x() + self.box_2d.size_x.get() / 2.0
        && p.y() >= self.box_2d.y() - self.box_2d.size_y.get() / 2.0
        && p.y() <= self.box_2d.y() + self.box_2d.size_y.get() / 2.0
    }
}

impl IsScalable for FilterBox2D {
    fn scale(&mut self, factor: Positive) {
        self.box_2d.scale(factor);
    }
}

impl From<BoundingBox2D> for FilterBox2D {
    fn from(x: BoundingBox2D) -> Self {
        Self::new(Box2D{center: x.center_bb(), size_x: x.size_x(), size_y: x.size_y()})
    }
}

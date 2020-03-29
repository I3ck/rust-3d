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

//! FilterCircle, a circle filter within 2D space

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Eq, Ord, Hash)]
/// FilterCircle, a circle filter within 2D space
pub struct FilterCircle {
    circle: Circle,
}
impl FilterCircle {
    /// Creates a new FilterCircle with the given parameters
    pub fn new(circle: Circle) -> Self {
        FilterCircle { circle }
    }
}

//------------------------------------------------------------------------------

impl IsND for FilterCircle {
    fn n_dimensions() -> usize {
        Circle::n_dimensions()
    }

    fn position_nd(&self, dimension: usize) -> Result<f64> {
        self.circle.position_nd(dimension)
    }
}

impl Is2D for FilterCircle {
    #[inline(always)]
    fn x(&self) -> f64 {
        self.circle.x()
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        self.circle.y()
    }
}

impl IsBuildableND for FilterCircle {
    #[inline(always)]
    fn new_nd(coords: &[f64]) -> Result<Self> {
        Ok(FilterCircle::new(Circle::new_nd(coords)?))
    }

    #[inline(always)]
    fn from_nd<P>(&mut self, other: P) -> Result<()>
    where
        P: IsBuildableND,
    {
        self.circle.from_nd(other)
    }
}

impl IsBuildable2D for FilterCircle {
    #[inline(always)]
    fn new(x: f64, y: f64) -> Self {
        FilterCircle::new(Circle::new(x, y))
    }

    #[inline(always)]
    fn from<P>(&mut self, other: &P)
    where
        P: Is2D,
    {
        self.circle.from(other)
    }
}

impl IsEditableND for FilterCircle {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        self.circle.set_position(dimension, val)
    }
}

impl IsEditable2D for FilterCircle {
    #[inline(always)]
    fn set_x(&mut self, val: f64) {
        self.circle.set_x(val);
    }

    #[inline(always)]
    fn set_y(&mut self, val: f64) {
        self.circle.set_y(val);
    }
}

impl HasBoundingBox2D for FilterCircle {
    fn bounding_box(&self) -> BoundingBox2D {
        self.circle.bounding_box()
    }
}

impl HasBoundingBox2DMaybe for FilterCircle {
    fn bounding_box_maybe(&self) -> Result<BoundingBox2D> {
        self.circle.bounding_box_maybe()
    }
}

impl<T> IsFilter<T> for FilterCircle
where
    T: Is2D,
{
    fn is_allowed(&self, p: &T) -> bool {
        dist_2d(p, &self.circle.center) <= self.circle.radius.get()
    }
}

impl IsScalable for FilterCircle {
    fn scale(&mut self, factor: Positive) {
        self.circle.scale(factor);
    }
}

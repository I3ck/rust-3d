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

//! Circle, a circle in 2D space

use std::cmp::{Eq, Ordering};

use crate::{distances_2d::*, prelude::*};

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash, Default)]
/// Circle, a circle in 2D space
pub struct Circle {
    pub center: Point2D,
    pub radius: Positive,
}

impl Eq for Circle {}

impl Ord for Circle {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point2D::default();
        match sqr_dist_2d(&origin, &self.center).partial_cmp(&sqr_dist_2d(&origin, &other.center)) {
            Some(x) => x,
            None => self
                .radius
                .partial_cmp(&other.radius)
                .unwrap_or(Ordering::Equal),
        }
    }
}

impl IsND for Circle {
    fn n_dimensions() -> usize {
        Point2D::n_dimensions()
    }

    fn position_nd(&self, dimension: usize) -> Result<f64> {
        self.center.position_nd(dimension)
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
        Ok(Circle {
            center: Point2D::new_nd(coords)?,
            radius: Positive::one(),
        })
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()>
    where
        P: IsBuildableND,
    {
        self.center.from_nd(other)
    }
}

impl IsBuildable2D for Circle {
    fn new(x: f64, y: f64) -> Self {
        Circle {
            center: Point2D { x: x, y: y },
            radius: Positive::one(),
        }
    }

    fn from<P>(&mut self, other: &P)
    where
        P: Is2D,
    {
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
    fn bounding_box(&self) -> BoundingBox2D {
        let p_min = Point2D {
            x: self.center.x() - self.radius.get(),
            y: self.center.y() - self.radius.get(),
        };
        let p_max = Point2D {
            x: self.center.x() + self.radius.get(),
            y: self.center.y() + self.radius.get(),
        };
        BoundingBox2D::new(&p_min, &p_max).unwrap() // safe
    }
}

impl HasBoundingBox2DMaybe for Circle {
    fn bounding_box_maybe(&self) -> Result<BoundingBox2D> {
        Ok(self.bounding_box())
    }
}

impl IsScalable for Circle {
    fn scale(&mut self, factor: Positive) {
        self.radius *= factor;
    }
}

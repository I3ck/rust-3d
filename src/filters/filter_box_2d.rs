/*
Copyright 2016 Martin Buck
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

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.box_2d.get_position(dimension)
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

    fn from<P>(&mut self, other: P) where
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
    fn bounding_box(&self) -> Result<BoundingBox2D> {
        self.box_2d.bounding_box()
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

impl From<BoundingBox2D> for FilterBox2D {
    fn from(x: BoundingBox2D) -> Self {
        Self::new(Box2D{center: x.center_bb(), size_x: x.size_x(), size_y: x.size_y()})
    }
}

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

//! FilterCircle, a circle filter within 2D space

use prelude::*;
use distances_2d::*;

#[derive (Debug, PartialEq, PartialOrd, Default, Clone, Eq, Ord, Hash)]
/// FilterCircle, a circle filter within 2D space
pub struct FilterCircle {
    circle: Circle
}
impl FilterCircle {
    /// Creates a new FilterCircle with the given parameters
    pub fn new(circle: Circle) -> Self {
        FilterCircle {circle: circle}
    }
}

impl IsND for FilterCircle {
    fn n_dimensions() -> usize {
        Circle::n_dimensions()
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.circle.get_position(dimension)
    }
}

impl Is2D for FilterCircle {
    fn x(&self) -> f64 {
        self.circle.x()
    }

    fn y(&self) -> f64 {
        self.circle.y()
    }
}

impl IsBuildableND for FilterCircle {
    fn new_nd(coords: &Vec<f64>) -> Result<Box<Self>> {
        if coords.len() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Box::new(FilterCircle::new(*Circle::new_nd(coords)?)))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        self.circle.from_nd(other)
    }
}

impl IsBuildable2D for FilterCircle {
    fn new(x: f64, y: f64) -> Box<Self> {
        Box::new(FilterCircle::new(*Circle::new(x, y)))
    }

    fn from<P>(&mut self, other: P)
        where P: Is2D {

        self.circle.from(other)
    }
}

impl IsEditableND for FilterCircle {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        self.circle.set_position(dimension, val)
    }
}

impl IsEditable2D for FilterCircle {
    fn set_x(&mut self, val: f64) {
        self.circle.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.circle.set_y(val);
    }
}

impl HasBoundingBox2D for FilterCircle {
    fn bounding_box(&self) -> Result<BoundingBox2D> {
        self.circle.bounding_box()
    }
}

impl<T> IsFilter<T> for FilterCircle where
    T: Is2D {

    fn is_allowed(&self, p: &T) -> bool {
        dist_2d(p, &self.circle.center) <= self.circle.radius.get()
    }
}

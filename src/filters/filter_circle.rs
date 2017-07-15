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

use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};

use result::*;
use traits::is_nd::*;
use traits::is_2d::*;
use traits::is_buildable_nd::*;
use traits::is_buildable_2d::*;
use traits::is_editable_nd::*;
use traits::is_editable_2d::*;
use traits::has_bounding_box_2d::*;
use traits::is_filter::*;
use point_2d::*;
use distances_2d::*;
use positive::*;
use bounding_box_2d::*;

#[derive (Debug, PartialEq, PartialOrd, Default, Clone)]
/// FilterCircle, a circle filter within 2D space
pub struct FilterCircle {
    center: Point2D,
    radius: Positive
}

impl Eq for FilterCircle {}

impl Ord for FilterCircle {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point2D::default();
        match sqr_dist_2d(&origin, &self.center).partial_cmp(&sqr_dist_2d(&origin, &other.center)) {
            Some(x) => x,
            None => self.radius.partial_cmp(&other.radius).unwrap_or(Ordering::Equal)
        }
    }
}

impl Hash for FilterCircle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        (self.radius.get() as u64).hash(state);
    }
}

impl FilterCircle {
    /// Creates a new FilterCircle with the given parameters
    pub fn new(center: Point2D, p_radius: Positive) -> Self {
        FilterCircle {center: center, radius: p_radius}
    }
}

impl IsND for FilterCircle {
    fn n_dimensions() -> usize {
        2
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.center.get_position(dimension)
    }
}

impl Is2D for FilterCircle {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }
}

impl IsBuildableND for FilterCircle {
    fn new_nd(coords: &Vec<f64>) -> Result<Box<Self>> {
        if coords.len() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Box::new(FilterCircle::new(Point2D{x: coords[0], y: coords[1]}, Positive::one())))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        if P::n_dimensions() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }

        self.center.set_x(other.get_position(0)?);
        self.center.set_y(other.get_position(1)?);
        Ok(())
    }
}

impl IsBuildable2D for FilterCircle {
    fn new(x: f64, y: f64) -> Box<Self> {
        Box::new(FilterCircle::new(Point2D{x: x, y: y}, Positive::one()))
    }

    fn from<P>(&mut self, other: P)
        where P: Is2D {

        self.center.from(other)
    }
}

impl IsEditableND for FilterCircle {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        match dimension {
            0 => self.center.set_x(val),
            1 => self.center.set_y(val),
            _ => return Err(ErrorKind::DimensionsDontMatch),
        }
        Ok(())
    }
}

impl IsEditable2D for FilterCircle {
    fn set_x(&mut self, val: f64) {
        self.center.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.center.set_y(val);
    }
}

impl HasBoundingBox2D for FilterCircle {
    fn bounding_box(&self) -> Result<BoundingBox2D> {
        let p_min = Point2D{x: self.center.x() - self.radius.get(), y: self.center.y() - self.radius.get()};
        let p_max = Point2D{x: self.center.x() + self.radius.get(), y: self.center.y() + self.radius.get()};
        BoundingBox2D::new(&p_min, &p_max)
    }
}

impl<T> IsFilter<T> for FilterCircle where
    T: Is2D {

    fn is_allowed(&self, p: &T) -> bool {
        dist_2d(p, &self.center) <= self.radius.get()
    }
}

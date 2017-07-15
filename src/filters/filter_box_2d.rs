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
/// FilterBox2D, a box filter within 2D space
pub struct FilterBox2D {
    center: Point2D,
    size_x: Positive,
    size_y: Positive
}

impl Eq for FilterBox2D {}

impl Ord for FilterBox2D {
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

impl Hash for FilterBox2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        (self.size_x.get() as u64).hash(state);
        (self.size_y.get() as u64).hash(state);
    }
}

impl FilterBox2D {
    /// Creates a new FilterBox2D with the given parameters
    pub fn new(center: Point2D, p_size_x: Positive, p_size_y: Positive) -> Self {
        FilterBox2D {center: center, size_x: p_size_x, size_y: p_size_y}
    }
}

impl IsND for FilterBox2D {
    fn n_dimensions() -> usize {
        2
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.center.get_position(dimension)
    }
}

impl Is2D for FilterBox2D {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }
}

impl IsBuildableND for FilterBox2D {
    fn new_nd(coords: &Vec<f64>) -> Result<Box<Self>> {
        if coords.len() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Box::new(FilterBox2D::new(*Point2D::new(coords[0], coords[1]), Positive::one(), Positive::one())))
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

impl IsBuildable2D for FilterBox2D {
    fn new(x: f64, y: f64) -> Box<Self> {
        Box::new(FilterBox2D::new(*Point2D::new(x, y), Positive::one(), Positive::one()))
    }

    fn from<P>(&mut self, other: P) where
        P: Is2D {

        self.center.from(other)
    }
}

impl IsEditableND for FilterBox2D {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        match dimension {
            0 => self.center.set_x(val),
            1 => self.center.set_y(val),
            _ => return Err(ErrorKind::DimensionsDontMatch),
        }
        Ok(())
    }
}

impl IsEditable2D for FilterBox2D {
    fn set_x(&mut self, val: f64) {
        self.center.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.center.set_y(val);
    }
}

impl HasBoundingBox2D for FilterBox2D {
    fn bounding_box(&self) -> Result<BoundingBox2D> {
        let p_min = *Point2D::new(self.center.x() - self.size_x.get() / 2.0, self.center.y() - self.size_y.get() / 2.0);
        let p_max = *Point2D::new(self.center.x() + self.size_x.get() / 2.0, self.center.y() + self.size_y.get() / 2.0);
        BoundingBox2D::new(&p_min, &p_max)
    }
}

impl<T> IsFilter<T> for FilterBox2D where
    T: Is2D {

    fn is_allowed(&self, p: &T) -> bool {
           p.x() >= self.center.x() - self.size_x.get() / 2.0
        && p.x() <= self.center.x() + self.size_x.get() / 2.0
        && p.y() >= self.center.y() - self.size_y.get() / 2.0
        && p.y() <= self.center.y() + self.size_y.get() / 2.0
    }
}

impl From<BoundingBox2D> for FilterBox2D {
    fn from(x: BoundingBox2D) -> Self {
        Self::new(x.center_bb(), x.size_x(), x.size_y())
    }
}

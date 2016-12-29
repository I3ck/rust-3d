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

use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};

use traits::is_nd::IsND;
use traits::is_2d::Is2D;
use traits::is_buildable_2d::IsBuildable2D;
use traits::is_editable_2d::IsEditable2D;
use traits::is_filter_2d::IsFilter2D;
use point_2d::Point2D;
use functions::{sqr_dist_2d};
use positive::Positive;

#[derive (PartialEq, PartialOrd)]
pub struct FilterBox2D {
    center: Point2D,
    size_x: f64,
    size_y: f64
}

impl Eq for FilterBox2D {}

impl Ord for FilterBox2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point2D::new();
        match sqr_dist_2d(&origin, &self.center).partial_cmp(&sqr_dist_2d(&origin, &other.center)) {
            Some(x) => x,
            None => match self.size_x.partial_cmp(&other.size_x) {
                Some(x) => x,
                None => self.size_y.partial_cmp(&other.size_y).unwrap_or(Ordering::Equal)
            }
        }
    }
}

impl Hash for FilterBox2D { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        (self.size_x as u64).hash(state);
        (self.size_y as u64).hash(state);
    }
}

impl FilterBox2D {
    fn new() -> Self {
        FilterBox2D {center: *Point2D::new(), size_x: 1.0, size_y: 1.0}
    }
    fn build(center: Point2D, p_size_x: Positive, p_size_y: Positive) -> Self {
        FilterBox2D {center: center, size_x: p_size_x.get(), size_y: p_size_y.get()}
    }
}

impl IsND for FilterBox2D {
    fn n_dimensions(&self) -> usize {
        2
    }

    fn get_position(&self, dimension: usize) -> Option<f64> {
        match dimension {
            0 => Some(self.center.x()),
            1 => Some(self.center.y()),
            _ => None
        }
    }
}

impl Is2D for FilterBox2D {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }

    fn clone(&self) -> FilterBox2D {
        FilterBox2D { center: self.center.clone(), size_x: self.size_x, size_y: self.size_y }
    }
}

//@todo drop this impl once not required anymore for editable?
//@todo or always set sizes to 1
impl IsBuildable2D for FilterBox2D {
    fn new() -> Box<Self> {
        Box::new(FilterBox2D::new())
    }

    fn build(x: f64, y: f64) -> Box<Self> {
        Box::new(FilterBox2D::build(*Point2D::build(x, y), Positive::build(1.0).unwrap(), Positive::build(1.0).unwrap()))
    }

    fn from<P>(&mut self, other: P) where P: IsBuildable2D {
        self.center.from(other)
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

impl<P> IsFilter2D<P> for FilterBox2D where
    P: Is2D {

    fn is_allowed(&self, p: &P) -> bool {
           p.x() >= self.center.x() - self.size_x / 2.0
        && p.x() <= self.center.x() + self.size_x / 2.0
        && p.y() >= self.center.y() - self.size_y / 2.0
        && p.y() <= self.center.y() + self.size_y / 2.0
    }
}

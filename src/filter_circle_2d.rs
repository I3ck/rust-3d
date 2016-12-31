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
use traits::has_bounding_box_2d::HasBoundingBox2D;
use traits::is_filter_2d::IsFilter2D;
use point_2d::Point2D;
use functions::{dist_2d, sqr_dist_2d};
use positive::Positive;

#[derive (PartialEq, PartialOrd)]
pub struct FilterCircle2D {
    center: Point2D,
    radius: f64
}

impl Eq for FilterCircle2D {}

impl Ord for FilterCircle2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point2D::new();
        match sqr_dist_2d(&origin, &self.center).partial_cmp(&sqr_dist_2d(&origin, &other.center)) {
            Some(x) => x,
            None => self.radius.partial_cmp(&other.radius).unwrap_or(Ordering::Equal)
        }
    }
}

impl Hash for FilterCircle2D { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        (self.radius as u64).hash(state);
    }
}

impl FilterCircle2D {
    fn new() -> Self {
        FilterCircle2D {center: *Point2D::new(), radius: 1.0}
    }
    fn build(center: Point2D, p_radius: Positive) -> Self {
        FilterCircle2D {center: center, radius: p_radius.get()}
    }
}

impl IsND for FilterCircle2D {
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

impl Is2D for FilterCircle2D {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }

    fn clone(&self) -> FilterCircle2D {
        FilterCircle2D { center: self.center.clone(), radius: self.radius }
    }
}

//@todo drop this impl once not required anymore for editable?
//@todo or always set sizes to 1
impl IsBuildable2D for FilterCircle2D {
    fn new() -> Box<Self> {
        Box::new(FilterCircle2D::new())
    }

    fn build(x: f64, y: f64) -> Box<Self> {
        Box::new(FilterCircle2D::build(*Point2D::build(x, y), Positive::build(1.0).unwrap()))
    }

    fn from<P>(&mut self, other: P) where P: IsBuildable2D {
        self.center.from(other)
    }
}

impl IsEditable2D for FilterCircle2D {
    fn set_x(&mut self, val: f64) {
        self.center.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.center.set_y(val);
    }
}

impl HasBoundingBox2D for FilterCircle2D {
    fn bounding_box(&self) -> Option<(Point2D, Point2D)> {
        if self.radius <= 0.0 {
            return None;
        }
        let p_min = *Point2D::build(self.center.x() - self.radius, self.center.y() - self.radius);
        let p_max = *Point2D::build(self.center.x() + self.radius, self.center.y() + self.radius);
        return Some((p_min, p_max));
    }
}

impl<P> IsFilter2D<P> for FilterCircle2D where
    P: Is2D {

    fn is_allowed(&self, p: &P) -> bool {
        dist_2d(p, &self.center) <= self.radius
    }
}

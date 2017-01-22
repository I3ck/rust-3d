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

use result::*;
use traits::is_nd::*;
use traits::is_2d::*;
use traits::is_buildable_nd::*;
use traits::is_buildable_2d::*;
use traits::is_editable_nd::*;
use traits::is_editable_2d::*;
use traits::has_bounding_box_2d::*;
use traits::is_filter_2d::*;
use point_2d::*;
use functions::{dist_2d, sqr_dist_2d};
use positive::*;

#[derive (PartialEq, PartialOrd)]
pub struct FilterCircle {
    center: Point2D,
    radius: f64
}

impl Eq for FilterCircle {}

impl Ord for FilterCircle {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point2D::new();
        match sqr_dist_2d(&origin, &self.center).partial_cmp(&sqr_dist_2d(&origin, &other.center)) {
            Some(x) => x,
            None => self.radius.partial_cmp(&other.radius).unwrap_or(Ordering::Equal)
        }
    }
}

impl Hash for FilterCircle { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        (self.radius as u64).hash(state);
    }
}

impl Clone for FilterCircle {
    fn clone(&self) -> FilterCircle {
        FilterCircle { center: self.center.clone(), radius: self.radius }
    }
}

impl FilterCircle {
    pub fn new() -> Self {
        FilterCircle {center: *Point2D::new(), radius: 1.0}
    }
    pub fn build(center: Point2D, p_radius: Positive) -> Self {
        FilterCircle {center: center, radius: p_radius.get()}
    }
}

impl IsND for FilterCircle {
    fn n_dimensions() -> usize {
        2
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        match dimension {
            0 => Ok(self.center.x()),
            1 => Ok(self.center.y()),
            _ => Err(ErrorKind::IncorrectDimension)
        }
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
    fn new() -> Box<Self> {
        Box::new(FilterCircle::new())
    }

    fn build_nd(coords: &Vec<f64>) -> Result<Box<Self>> {
        if coords.len() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Box::new(FilterCircle::build(*Point2D::build(coords[0], coords[1]), Positive::new(1.0).unwrap())))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        if P::n_dimensions() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }

        self.center.set_x(try!(other.get_position(0)));
        self.center.set_y(try!(other.get_position(1)));
        Ok(())
    }
}

//@todo drop this impl once not required anymore for editable?
//@todo or always set sizes to 1
impl IsBuildable2D for FilterCircle {
    fn build(x: f64, y: f64) -> Box<Self> {
        Box::new(FilterCircle::build(*Point2D::build(x, y), Positive::new(1.0).unwrap()))
    }

    fn from<P>(&mut self, other: P) where P: IsBuildable2D {
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
    fn bounding_box(&self) -> Result<(Point2D, Point2D)> {
        let p_min = *Point2D::build(self.center.x() - self.radius, self.center.y() - self.radius);
        let p_max = *Point2D::build(self.center.x() + self.radius, self.center.y() + self.radius);
        return Ok((p_min, p_max));
    }
}

impl IsFilter2D for FilterCircle {
    fn is_allowed(&self, p: &Is2D) -> bool {
        dist_2d(p, &self.center) <= self.radius
    }
}

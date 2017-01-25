/*
Copyright 2017 Martin Buck
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

//! Module containing FilterSphere, a sphere filter within 3D space

use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};

use result::*;
use traits::is_nd::*;
use traits::is_3d::*;
use traits::is_buildable_nd::*;
use traits::is_buildable_3d::*;
use traits::is_editable_nd::*;
use traits::is_editable_3d::*;
use traits::has_bounding_box_3d::*;
use traits::is_filter_3d::*;
use point_3d::*;
use functions::{dist_3d, sqr_dist_3d};
use positive::*;

#[derive (PartialEq, PartialOrd)]
/// FilterSphere, a sphere filter within 3D space
pub struct FilterSphere {
    center: Point3D,
    radius: f64
}

impl Eq for FilterSphere {}

impl Ord for FilterSphere {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point3D::new();
        match sqr_dist_3d(&origin, &self.center).partial_cmp(&sqr_dist_3d(&origin, &other.center)) {
            Some(x) => x,
            None => self.radius.partial_cmp(&other.radius).unwrap_or(Ordering::Equal)
        }
    }
}

impl Hash for FilterSphere { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        (self.radius as u64).hash(state);
    }
}

impl Clone for FilterSphere {
    fn clone(&self) -> FilterSphere {
        FilterSphere { center: self.center.clone(), radius: self.radius }
    }
}

impl FilterSphere {
    /// Creates a new FilterSphere at origin with a radius of 1
    pub fn new() -> Self {
        FilterSphere {center: *Point3D::new(), radius: 1.0}
    }
    /// Creates a new FilterSphere with the given parameters
    pub fn build(center: Point3D, p_radius: Positive) -> Self {
        FilterSphere {center: center, radius: p_radius.get()}
    }
}

impl IsND for FilterSphere {
    fn n_dimensions() -> usize {
        3
    }

    fn get_position(&self, dimension: usize) -> Result<f64> { //@todo here and for many others could be reworked to simply return get_position of center
        match dimension {
            0 => Ok(self.center.x()),
            1 => Ok(self.center.y()),
            2 => Ok(self.center.z()),
            _ => Err(ErrorKind::IncorrectDimension)
        }
    }
}

impl Is3D for FilterSphere {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }

    fn z(&self) -> f64 {
        self.center.y()
    }
}

impl IsBuildableND for FilterSphere {
    fn new() -> Box<Self> {
        Box::new(FilterSphere::new())
    }

    fn build_nd(coords: &Vec<f64>) -> Result<Box<Self>> {
        if coords.len() != 3 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Box::new(FilterSphere::build(*Point3D::build(coords[0], coords[1], coords[2]), Positive::new(1.0).unwrap())))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        if P::n_dimensions() != 3 {
            return Err(ErrorKind::DimensionsDontMatch);
        }

        self.center.set_x(try!(other.get_position(0)));
        self.center.set_y(try!(other.get_position(1)));
        self.center.set_z(try!(other.get_position(2)));
        Ok(())
    }
}

//@todo drop this impl once not required anymore for editable?
//@todo or always set sizes to 1
impl IsBuildable3D for FilterSphere {
    fn build(x: f64, y: f64, z: f64) -> Box<Self> {
        Box::new(FilterSphere::build(*Point3D::build(x, y, z), Positive::new(1.0).unwrap()))
    }

    fn from<P>(&mut self, other: P) where P: IsBuildable3D {
        self.center.from(other)
    }
}

impl IsEditableND for FilterSphere {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        match dimension {
            0 => self.center.set_x(val),
            1 => self.center.set_y(val),
            2 => self.center.set_z(val),
            _ => return Err(ErrorKind::DimensionsDontMatch),
        }
        Ok(())
    }
}

impl IsEditable3D for FilterSphere {
    fn set_x(&mut self, val: f64) {
        self.center.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.center.set_y(val);
    }

    fn set_z(&mut self, val: f64) {
        self.center.set_z(val);
    }
}

impl HasBoundingBox3D for FilterSphere {
    fn bounding_box(&self) -> Result<(Point3D, Point3D)> {
        let p_min = *Point3D::build(self.center.x() - self.radius, self.center.y() - self.radius, self.center.z() - self.radius);
        let p_max = *Point3D::build(self.center.x() + self.radius, self.center.y() + self.radius, self.center.z() + self.radius);
        return Ok((p_min, p_max));
    }
}

impl IsFilter3D for FilterSphere {
    fn is_allowed(&self, p: &Is3D) -> bool {
        dist_3d(p, &self.center) <= self.radius
    }
}

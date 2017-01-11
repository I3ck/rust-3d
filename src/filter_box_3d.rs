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

use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};

use traits::is_nd::IsND;
use traits::is_3d::Is3D;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_editable_3d::IsEditable3D;
use traits::has_bounding_box_3d::HasBoundingBox3D;
use traits::is_filter_3d::IsFilter3D;
use point_3d::Point3D;
use functions::{sqr_dist_3d};
use positive::Positive;

#[derive (PartialEq, PartialOrd)]
pub struct FilterBox3D {
    center: Point3D,
    size_x: f64,
    size_y: f64,
    size_z: f64
}

impl Eq for FilterBox3D {}

impl Ord for FilterBox3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point3D::new();
        match sqr_dist_3d(&origin, &self.center).partial_cmp(&sqr_dist_3d(&origin, &other.center)) {
            Some(x) => x,
            None => match self.size_x.partial_cmp(&other.size_x) {
                Some(x) => x,
                None => match self.size_y.partial_cmp(&other.size_y) {
                    Some(x) => x,
                    None => self.size_z.partial_cmp(&other.size_z).unwrap_or(Ordering::Equal)
                }
            }
        }
    }
}

impl Hash for FilterBox3D { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        (self.size_x as u64).hash(state);
        (self.size_y as u64).hash(state);
        (self.size_z as u64).hash(state);
    }
}

impl Clone for FilterBox3D {
    fn clone(&self) -> FilterBox3D {
        FilterBox3D { center: self.center.clone(), size_x: self.size_x, size_y: self.size_y, size_z: self.size_z }
    }
}

impl FilterBox3D {
    pub fn new() -> Self {
        FilterBox3D {center: *Point3D::new(), size_x: 1.0, size_y: 1.0, size_z: 1.0}
    }
    pub fn build(center: Point3D, p_size_x: Positive, p_size_y: Positive, p_size_z: Positive) -> Self {
        FilterBox3D {center: center, size_x: p_size_x.get(), size_y: p_size_y.get(), size_z: p_size_z.get()}
    }
    pub fn from_bb(hbb: &HasBoundingBox3D) -> Option<Self> {
        match (hbb.center_bb(), hbb.size_x(), hbb.size_y(), hbb.size_z()) {
            (Some(center), Some(sx), Some(sy), Some(sz)) => if sx > 0.0 && sy > 0.0 && sz > 0.0 {
                    Some(Self::build(center, Positive::build(sx).unwrap(), Positive::build(sy).unwrap(), Positive::build(sz).unwrap()))
                } else {
                    None
                },
            _ => None,
        }
    }
}

impl IsND for FilterBox3D {
    fn n_dimensions(&self) -> usize {
        3
    }

    fn get_position(&self, dimension: usize) -> Option<f64> {
        match dimension {
            0 => Some(self.center.x()),
            1 => Some(self.center.y()),
            2 => Some(self.center.z()),
            _ => None
        }
    }
}

impl Is3D for FilterBox3D {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }

    fn z(&self) -> f64 {
        self.center.z()
    }
}

//@todo drop this impl once not required anymore for editable?
//@todo or always set sizes to 1
impl IsBuildable3D for FilterBox3D {
    fn new() -> Box<Self> {
        Box::new(FilterBox3D::new())
    }

    fn build(x: f64, y: f64, z: f64) -> Box<Self> {
        Box::new(FilterBox3D::build(*Point3D::build(x, y, z), Positive::build(1.0).unwrap(), Positive::build(1.0).unwrap(), Positive::build(1.0).unwrap()))
    }

    fn from<P>(&mut self, other: P) where P: IsBuildable3D {
        self.center.from(other)
    }
}

impl IsEditable3D for FilterBox3D {
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

impl HasBoundingBox3D for FilterBox3D {
    fn bounding_box(&self) -> Option<(Point3D, Point3D)> {
        if self.size_x <= 0.0 || self.size_y <= 0.0 || self.size_z <= 0.0 { //@todo can be dropped here and in 2D version, since size is now a positive
            return None;
        }
        let p_min = *Point3D::build(self.center.x() - self.size_x / 2.0, self.center.y() - self.size_y / 2.0, self.center.z() - self.size_z / 2.0);
        let p_max = *Point3D::build(self.center.x() + self.size_x / 2.0, self.center.y() + self.size_y / 2.0, self.center.z() + self.size_z / 2.0);
        return Some((p_min, p_max));
    }
}

impl IsFilter3D for FilterBox3D {
    fn is_allowed(&self, p: &Is3D) -> bool {
           p.x() >= self.center.x() - self.size_x / 2.0
        && p.x() <= self.center.x() + self.size_x / 2.0
        && p.y() >= self.center.y() - self.size_y / 2.0
        && p.y() <= self.center.y() + self.size_y / 2.0
        && p.z() >= self.center.z() - self.size_z / 2.0
        && p.z() <= self.center.z() + self.size_z / 2.0
    }
}

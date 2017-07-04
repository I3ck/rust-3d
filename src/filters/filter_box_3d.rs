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

//! FilterBox3D, a box filter within 3D space

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
use traits::is_filter::*;
use point_3d::*;
use functions::{sqr_dist_3d};
use positive::*;
use bounding_box_3d::*;

#[derive (PartialEq, PartialOrd, Default)]
/// FilterBox3D, a box filter within 3D space
pub struct FilterBox3D {
    center: Point3D,
    size_x: Positive,
    size_y: Positive,
    size_z: Positive
}

impl Eq for FilterBox3D {}

impl Ord for FilterBox3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point3D::default();
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
        (self.size_x.get() as u64).hash(state);
        (self.size_y.get() as u64).hash(state);
        (self.size_z.get() as u64).hash(state);
    }
}

impl Clone for FilterBox3D {
    fn clone(&self) -> FilterBox3D {
        FilterBox3D { center: self.center.clone(), size_x: self.size_x, size_y: self.size_y, size_z: self.size_z }
    }
}

impl FilterBox3D {
    /// Creates a new FilterBox3D with the given parameters
    pub fn new(center: Point3D, p_size_x: Positive, p_size_y: Positive, p_size_z: Positive) -> Self {
        FilterBox3D {center: center, size_x: p_size_x, size_y: p_size_y, size_z: p_size_z}
    }
}

impl IsND for FilterBox3D {
    fn n_dimensions() -> usize {
        3
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        match dimension {
            0 => Ok(self.center.x()),
            1 => Ok(self.center.y()),
            2 => Ok(self.center.z()),
            _ => Err(ErrorKind::IncorrectDimension)
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

impl IsBuildableND for FilterBox3D {
    fn new_nd(coords: &Vec<f64>) -> Result<Box<Self>> {
        if coords.len() != 3 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Box::new(FilterBox3D::new(*Point3D::new(coords[0], coords[1], coords[2]), Positive::one(), Positive::one(), Positive::one())))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        if P::n_dimensions() != 3 {
            return Err(ErrorKind::DimensionsDontMatch);
        }

        self.center.set_x(other.get_position(0)?);
        self.center.set_y(other.get_position(1)?);
        self.center.set_z(other.get_position(2)?);
        Ok(())
    }
}

impl IsBuildable3D for FilterBox3D {
    fn new(x: f64, y: f64, z: f64) -> Box<Self> {
        Box::new(FilterBox3D::new(*Point3D::new(x, y, z), Positive::one(), Positive::one(), Positive::one()))
    }

    fn from<P>(&mut self, other: P)
        where P: Is3D {

        self.center.from(other)
    }
}

impl IsEditableND for FilterBox3D {
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
    fn bounding_box(&self) -> Result<BoundingBox3D> {
        let p_min = *Point3D::new(self.center.x() - self.size_x.get() / 2.0, self.center.y() - self.size_y.get() / 2.0, self.center.z() - self.size_z.get() / 2.0);
        let p_max = *Point3D::new(self.center.x() + self.size_x.get() / 2.0, self.center.y() + self.size_y.get() / 2.0, self.center.z() + self.size_z.get() / 2.0);
        BoundingBox3D::new(p_min, p_max)
    }
}

impl<T> IsFilter<T> for FilterBox3D where
    T: Is3D {

    fn is_allowed(&self, p: &T) -> bool {
           p.x() >= self.center.x() - self.size_x.get() / 2.0
        && p.x() <= self.center.x() + self.size_x.get() / 2.0
        && p.y() >= self.center.y() - self.size_y.get() / 2.0
        && p.y() <= self.center.y() + self.size_y.get() / 2.0
        && p.z() >= self.center.z() - self.size_z.get() / 2.0
        && p.z() <= self.center.z() + self.size_z.get() / 2.0
    }
}

impl From<BoundingBox3D> for FilterBox3D {
    fn from(x: BoundingBox3D) -> Self {
        Self::new(x.center_bb(), x.size_x(), x.size_y(), x.size_z())
    }
}

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

use prelude::*;

#[derive (Debug, PartialEq, PartialOrd, Default, Clone, Eq, Hash, Ord)]
/// FilterBox3D, a box filter within 3D space
pub struct FilterBox3D {
    box_3d: Box3D
}

impl FilterBox3D {
    /// Creates a new FilterBox3D with the given parameters
    pub fn new(box_3d: Box3D) -> Self {
        FilterBox3D {box_3d: box_3d}
    }
}

impl IsND for FilterBox3D {
    fn n_dimensions() -> usize {
        Box3D::n_dimensions()
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.box_3d.get_position(dimension)
    }
}

impl Is3D for FilterBox3D {
    fn x(&self) -> f64 {
        self.box_3d.x()
    }

    fn y(&self) -> f64 {
        self.box_3d.y()
    }

    fn z(&self) -> f64 {
        self.box_3d.z()
    }
}

impl IsBuildableND for FilterBox3D {
    fn new_nd(coords: &Vec<f64>) -> Result<Box<Self>> {
        if coords.len() != 3 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Box::new(FilterBox3D::new(*Box3D::new_nd(coords)?)))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        self.box_3d.from_nd(other)
    }
}

impl IsBuildable3D for FilterBox3D {
    fn new(x: f64, y: f64, z: f64) -> Box<Self> {
        Box::new(FilterBox3D::new(*Box3D::new(x, y, z)))
    }

    fn from<P>(&mut self, other: P)
        where P: Is3D {

        self.box_3d.from(other)
    }
}

impl IsEditableND for FilterBox3D {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        self.box_3d.set_position(dimension, val)
    }
}

impl IsEditable3D for FilterBox3D {
    fn set_x(&mut self, val: f64) {
        self.box_3d.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.box_3d.set_y(val);
    }

    fn set_z(&mut self, val: f64) {
        self.box_3d.set_z(val);
    }
}

impl HasBoundingBox3D for FilterBox3D {
    fn bounding_box(&self) -> Result<BoundingBox3D> {
        self.box_3d.bounding_box()
    }
}

impl<T> IsFilter<T> for FilterBox3D where
    T: Is3D {

    fn is_allowed(&self, p: &T) -> bool {
           p.x() >= self.box_3d.x() - self.box_3d.size_x.get() / 2.0
        && p.x() <= self.box_3d.x() + self.box_3d.size_x.get() / 2.0
        && p.y() >= self.box_3d.y() - self.box_3d.size_y.get() / 2.0
        && p.y() <= self.box_3d.y() + self.box_3d.size_y.get() / 2.0
        && p.z() >= self.box_3d.z() - self.box_3d.size_z.get() / 2.0
        && p.z() <= self.box_3d.z() + self.box_3d.size_z.get() / 2.0
    }
}

impl From<BoundingBox3D> for FilterBox3D {
    fn from(x: BoundingBox3D) -> Self {
        Self::new(Box3D{center: x.center_bb(), size_x: x.size_x(), size_y: x.size_y(), size_z: x.size_z()})
    }
}

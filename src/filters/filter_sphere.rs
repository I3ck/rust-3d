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

//! FilterSphere, a sphere filter within 3D space

use prelude::*;
use distances_3d::*;

#[derive (Debug, PartialEq, PartialOrd, Default, Clone, Hash, Eq, Ord)]
/// FilterSphere, a sphere filter within 3D space
pub struct FilterSphere {
    sphere: Sphere
}

impl FilterSphere {
    /// Creates a new FilterSphere with the given parameters
    pub fn new(sphere: Sphere) -> Self {
        FilterSphere {sphere: sphere}
    }
}

impl IsND for FilterSphere {
    fn n_dimensions() -> usize {
        Sphere::n_dimensions()
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.sphere.get_position(dimension)
    }
}

impl Is3D for FilterSphere {
    fn x(&self) -> f64 {
        self.sphere.x()
    }

    fn y(&self) -> f64 {
        self.sphere.y()
    }

    fn z(&self) -> f64 {
        self.sphere.y()
    }
}

impl IsBuildableND for FilterSphere {
    fn new_nd(coords: &Vec<f64>) -> Result<Self> {
        Ok(FilterSphere::new(Sphere::new_nd(coords)?))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        self.sphere.from_nd(other)
    }
}

impl IsBuildable3D for FilterSphere {
    fn new(x: f64, y: f64, z: f64) -> Self {
        FilterSphere::new(Sphere::new(x, y, z))
    }

    fn from<P>(&mut self, other: P)
        where P: Is3D {

        self.sphere.from(other)
    }
}

impl IsEditableND for FilterSphere {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        self.sphere.set_position(dimension, val)
    }
}

impl IsEditable3D for FilterSphere {
    fn set_x(&mut self, val: f64) {
        self.sphere.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.sphere.set_y(val);
    }

    fn set_z(&mut self, val: f64) {
        self.sphere.set_z(val);
    }
}

impl HasBoundingBox3D for FilterSphere {
    fn bounding_box(&self) -> Result<BoundingBox3D> {
        self.sphere.bounding_box()
    }
}

impl<T> IsFilter<T> for FilterSphere
    where T: Is3D {

    fn is_allowed(&self, p: &T) -> bool {
        dist_3d(p, &self.sphere.center) <= self.sphere.radius.get()
    }
}

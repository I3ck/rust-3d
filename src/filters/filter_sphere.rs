/*
Copyright 2017 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
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
        FilterSphere {sphere}
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
    fn new_nd(coords: &[f64]) -> Result<Self> {
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

impl IsScalable for FilterSphere {
    fn scale(&mut self, factor: Positive) {
        self.sphere.scale(factor);
    }
}

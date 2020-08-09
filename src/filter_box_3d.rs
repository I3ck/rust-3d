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

//! FilterBox3D, a box filter within 3D space

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd, Default, Clone, Eq, Hash, Ord)]
/// FilterBox3D, a box filter within 3D space
pub struct FilterBox3D {
    box_3d: Box3D,
}

impl FilterBox3D {
    /// Creates a new FilterBox3D with the given parameters
    pub fn new(box_3d: Box3D) -> Self {
        FilterBox3D { box_3d }
    }
}

//------------------------------------------------------------------------------

impl IsND for FilterBox3D {
    fn n_dimensions() -> usize {
        Box3D::n_dimensions()
    }

    fn position_nd(&self, dimension: usize) -> Option<f64> {
        self.box_3d.position_nd(dimension)
    }
}

impl Is3D for FilterBox3D {
    #[inline(always)]
    fn x(&self) -> f64 {
        self.box_3d.x()
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        self.box_3d.y()
    }

    #[inline(always)]
    fn z(&self) -> f64 {
        self.box_3d.z()
    }
}

impl IsBuildableND for FilterBox3D {
    #[inline(always)]
    fn new_nd(coords: &[f64]) -> Result<Self> {
        Ok(FilterBox3D::new(Box3D::new_nd(coords)?))
    }

    #[inline(always)]
    fn from_nd<P>(&mut self, other: P) -> Result<()>
    where
        P: IsBuildableND,
    {
        self.box_3d.from_nd(other)
    }
}

impl IsBuildable3D for FilterBox3D {
    #[inline(always)]
    fn new(x: f64, y: f64, z: f64) -> Self {
        FilterBox3D::new(Box3D::new(x, y, z))
    }

    #[inline(always)]
    fn from<P>(&mut self, other: &P)
    where
        P: Is3D,
    {
        self.box_3d.from(other)
    }
}

impl IsEditableND for FilterBox3D {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        self.box_3d.set_position(dimension, val)
    }
}

impl IsEditable3D for FilterBox3D {
    #[inline(always)]
    fn set_x(&mut self, val: f64) {
        self.box_3d.set_x(val);
    }

    #[inline(always)]
    fn set_y(&mut self, val: f64) {
        self.box_3d.set_y(val);
    }

    #[inline(always)]
    fn set_z(&mut self, val: f64) {
        self.box_3d.set_z(val);
    }
}

impl HasBoundingBox3D for FilterBox3D {
    fn bounding_box(&self) -> BoundingBox3D {
        self.box_3d.bounding_box()
    }
}

impl HasBoundingBox3DMaybe for FilterBox3D {
    fn bounding_box_maybe(&self) -> Result<BoundingBox3D> {
        self.box_3d.bounding_box_maybe()
    }
}

impl<T> IsFilter<T> for FilterBox3D
where
    T: Is3D,
{
    fn is_allowed(&self, p: &T) -> bool {
        p.x() >= self.box_3d.x() - self.box_3d.size_x.get() / 2.0
            && p.x() <= self.box_3d.x() + self.box_3d.size_x.get() / 2.0
            && p.y() >= self.box_3d.y() - self.box_3d.size_y.get() / 2.0
            && p.y() <= self.box_3d.y() + self.box_3d.size_y.get() / 2.0
            && p.z() >= self.box_3d.z() - self.box_3d.size_z.get() / 2.0
            && p.z() <= self.box_3d.z() + self.box_3d.size_z.get() / 2.0
    }
}

impl IsScalable for FilterBox3D {
    fn scale(&mut self, factor: Positive) {
        self.box_3d.scale(factor);
    }
}

impl From<BoundingBox3D> for FilterBox3D {
    fn from(x: BoundingBox3D) -> Self {
        Self::new(Box3D {
            center: x.center_bb(),
            size_x: x.size_x(),
            size_y: x.size_y(),
            size_z: x.size_z(),
        })
    }
}

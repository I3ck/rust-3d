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

//! Box3D, a box in 3D space

use std::cmp::{Eq, Ordering};

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash, Default)]
/// Box3D, a box in 3D space
pub struct Box3D {
    pub center: Point3D,
    pub size_x: Positive,
    pub size_y: Positive,
    pub size_z: Positive,
}

impl Box3D {
    /// Returns the minimum position of the box
    pub fn min_p(&self) -> Point3D {
        Point3D::new(
            self.center.x() - 0.5 * self.size_x.get(),
            self.center.y() - 0.5 * self.size_y.get(),
            self.center.z() - 0.5 * self.size_z.get(),
        )
    }
    /// Returns the maximum position of the box
    pub fn max_p(&self) -> Point3D {
        Point3D::new(
            self.center.x() + 0.5 * self.size_x.get(),
            self.center.y() + 0.5 * self.size_y.get(),
            self.center.z() + 0.5 * self.size_z.get(),
        )
    }
    /// Returns the sizes of the bounding box
    pub fn sizes(&self) -> [Positive; 3] {
        [self.size_x, self.size_y, self.size_z]
    }
}

//------------------------------------------------------------------------------

impl Eq for Box3D {}

impl Ord for Box3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point3D::default();
        match sqr_dist_3d(&origin, &self.center).partial_cmp(&sqr_dist_3d(&origin, &other.center)) {
            Some(x) => x,
            None => match self.size_x.partial_cmp(&other.size_x) {
                Some(x) => x,
                None => match self.size_y.partial_cmp(&other.size_y) {
                    Some(x) => x,
                    None => self
                        .size_z
                        .partial_cmp(&other.size_z)
                        .unwrap_or(Ordering::Equal),
                },
            },
        }
    }
}

impl IsND for Box3D {
    fn n_dimensions() -> usize {
        Point3D::n_dimensions()
    }

    fn position_nd(&self, dimension: usize) -> Option<f64> {
        self.center.position_nd(dimension)
    }
}

impl Is3D for Box3D {
    #[inline(always)]
    fn x(&self) -> f64 {
        self.center.x()
    }

    #[inline(always)]
    fn y(&self) -> f64 {
        self.center.y()
    }

    #[inline(always)]
    fn z(&self) -> f64 {
        self.center.z()
    }
}

impl IsBuildableND for Box3D {
    #[inline(always)]
    fn new_nd(coords: &[f64]) -> Result<Self> {
        Ok(Box3D {
            center: Point3D::new_nd(coords)?,
            size_x: Positive::one(),
            size_y: Positive::one(),
            size_z: Positive::one(),
        })
    }

    #[inline(always)]
    fn from_nd<P>(&mut self, other: P) -> Result<()>
    where
        P: IsBuildableND,
    {
        self.center.from_nd(other)
    }
}

impl IsBuildable3D for Box3D {
    #[inline(always)]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Box3D {
            center: Point3D { x, y, z },
            size_x: Positive::one(),
            size_y: Positive::one(),
            size_z: Positive::one(),
        }
    }

    #[inline(always)]
    fn from<P>(&mut self, other: &P)
    where
        P: Is3D,
    {
        self.center.from(other)
    }
}

impl IsEditableND for Box3D {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        self.center.set_position(dimension, val)
    }
}

impl IsEditable3D for Box3D {
    #[inline(always)]
    fn set_x(&mut self, val: f64) {
        self.center.set_x(val);
    }

    #[inline(always)]
    fn set_y(&mut self, val: f64) {
        self.center.set_y(val);
    }

    #[inline(always)]
    fn set_z(&mut self, val: f64) {
        self.center.set_z(val);
    }
}

impl HasBoundingBox3D for Box3D {
    fn bounding_box(&self) -> BoundingBox3D {
        let p_min = Point3D {
            x: self.center.x() - self.size_x.get() / 2.0,
            y: self.center.y() - self.size_y.get() / 2.0,
            z: self.center.z() - self.size_z.get() / 2.0,
        };
        let p_max = Point3D {
            x: self.center.x() + self.size_x.get() / 2.0,
            y: self.center.y() + self.size_y.get() / 2.0,
            z: self.center.z() + self.size_z.get() / 2.0,
        };
        BoundingBox3D::new(&p_min, &p_max).unwrap() // safe
    }
}

impl HasBoundingBox3DMaybe for Box3D {
    fn bounding_box_maybe(&self) -> Result<BoundingBox3D> {
        Ok(self.bounding_box())
    }
}

impl IsScalable for Box3D {
    fn scale(&mut self, factor: Positive) {
        self.size_x *= factor;
        self.size_y *= factor;
        self.size_z *= factor;
    }
}

impl IsMovable3D for Box3D {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.center.move_by(x, y, z)
    }
}

impl From<BoundingBox3D> for Box3D {
    fn from(x: BoundingBox3D) -> Self {
        Box3D {
            center: x.center_bb(),
            size_x: x.size_x(),
            size_y: x.size_y(),
            size_z: x.size_z(),
        }
    }
}

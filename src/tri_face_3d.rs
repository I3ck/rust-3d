/*
Copyright 2019 Martin Buck

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

//! Face with 3 corners in 3D space

use crate::*;

//------------------------------------------------------------------------------

/// Face with 3 corners in 3D space
pub struct TriFace3D {
    a: Point3D,
    b: Point3D,
    c: Point3D,
}

//------------------------------------------------------------------------------

impl TriFace3D {
    pub fn new(a: Point3D, b: Point3D, c: Point3D) -> Result<Self> {
        match BoundingBox3D::from_iterator([&a, &b, &c].iter().map(|x| *x)) {
            Err(_) => Err(ErrorKind::TriFace3DNotSpanningVolume),
            Ok(_) => Ok(Self { a, b, c }),
        }
    }

    pub fn a(&self) -> &Point3D {
        &self.a
    }

    pub fn b(&self) -> &Point3D {
        &self.b
    }

    pub fn c(&self) -> &Point3D {
        &self.c
    }
}

//------------------------------------------------------------------------------

impl IsSATObject for TriFace3D {
    fn for_each_point<F>(&self, f: &mut F)
    where
        F: FnMut(&Point3D),
    {
        f(&self.a);
        f(&self.b);
        f(&self.c);
    }

    fn for_each_axis<F>(&self, f: &mut F)
    where
        F: FnMut(&Norm3D),
    {
        let vab = conn(&self.a, &self.b);
        let vbc = conn(&self.b, &self.c);
        let vca = conn(&self.c, &self.a);

        let n = Norm3D::new(cross(&vab, &vbc)).unwrap_or(Norm3D::norm_z());
        let e1 = Norm3D::new(cross(&n, &vab)).unwrap_or(Norm3D::norm_z());
        let e2 = Norm3D::new(cross(&n, &vbc)).unwrap_or(Norm3D::norm_z());
        let e3 = Norm3D::new(cross(&n, &vca)).unwrap_or(Norm3D::norm_z());

        f(&n);
        f(&e1);
        f(&e2);
        f(&e3);
    }
}

//------------------------------------------------------------------------------

impl HasBoundingBox3D for TriFace3D {
    fn bounding_box(&self) -> BoundingBox3D {
        BoundingBox3D::from_iterator([&self.a, &self.b, &self.c].iter().map(|x| *x)).unwrap()
        // safe since ensured in constructor
    }
}

//------------------------------------------------------------------------------

impl HasBoundingBox3DMaybe for TriFace3D {
    fn bounding_box_maybe(&self) -> Option<BoundingBox3D> {
        Some(self.bounding_box())
    }
}

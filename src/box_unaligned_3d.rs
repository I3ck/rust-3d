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

//! Not axis aligned Box in 3D space

use crate::*;

//------------------------------------------------------------------------------

/// Not axis aligned Box in 3D space
#[derive(Clone)]
pub struct BoxUnaligned3D {
    //@todo better name (Box3D vs BoxUnaligned3D vs BoundingBox3D)
    pub center: Point3D,
    pub y_dir: Norm3D,
    pub z_dir: Norm3D,
    pub size: [Positive; 3],
}

//------------------------------------------------------------------------------

impl BoxUnaligned3D {
    pub fn new_from_bb(bb: &BoundingBox3D) -> Self {
        let center = Point3D::new_from(&bb.center_bb());
        let y_dir = Norm3D::norm_y();
        let z_dir = Norm3D::norm_z();
        let size = [bb.size_x(), bb.size_y(), bb.size_z()];

        Self {
            center,
            y_dir,
            z_dir,
            size,
        }
    }
    pub fn new_from_z_rotation<P>(center: &P, size: [Positive; 3], rotation: f64) -> Self
    where
        P: Is3D,
    {
        let mut y_dir = Point3D::new(0.0, 1.0, 0.0);
        let z_dir = Norm3D::norm_z();

        y_dir = rot2d(&y_dir, rotation);
        Self {
            center: Point3D::new_from(center),
            size,
            y_dir: Norm3D::new(y_dir).unwrap(), // safe since rotation of valid length
            z_dir,
        }
    }
    fn d_x(&self) -> Point3D {
        let d = self.size[0].get();
        Point3D::new_from(&self.x_dir()) * d
    }
    fn d_y(&self) -> Point3D {
        let d = self.size[1].get();
        Point3D::new_from(&self.y_dir) * d
    }
    fn d_z(&self) -> Point3D {
        let d = self.size[2].get();
        Point3D::new_from(&self.z_dir) * d
    }
    pub fn x_dir(&self) -> Norm3D {
        Norm3D::new(cross(&self.z_dir, &Point3D::new_from(&self.y_dir))).unwrap_or(Norm3D::norm_z())
    }
}

//------------------------------------------------------------------------------

impl IsMovable3D for BoxUnaligned3D {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.center.move_by(x, y, z);
    }
}

//------------------------------------------------------------------------------

impl IsSATObject for BoxUnaligned3D {
    //@todo also implement for Box3D
    fn for_each_point<F>(&self, f: &mut F)
    where
        F: FnMut(&Point3D),
    {
        let c = &self.center;
        let dx = &Point3D::new_from(&self.d_x());
        let dy = &Point3D::new_from(&self.d_y());
        let dz = &Point3D::new_from(&self.d_z());

        f(&(c - dx - (dy - dz)));
        f(&(c - dx - (dy + dz)));
        f(&(c - dx + (dy - dz)));
        f(&(c - dx + (dy + dz)));
        f(&(c + dx - (dy - dz)));
        f(&(c + dx - (dy + dz)));
        f(&(c + dx + (dy - dz)));
        f(&(c + dx + (dy + dz)));
    }

    fn for_each_axis<F>(&self, f: &mut F)
    where
        F: FnMut(&Norm3D),
    {
        f(&self.x_dir());
        f(&self.y_dir);
        f(&self.z_dir);
    }
}

//------------------------------------------------------------------------------

impl HasBoundingBox3D for BoxUnaligned3D {
    fn bounding_box(&self) -> BoundingBox3D {
        let mut max_size = self.size[0];
        if self.size[1] > max_size {
            max_size = self.size[1]
        }
        if self.size[2] > max_size {
            max_size = self.size[2]
        }

        BoundingBox3D::new(
            &Point3D::new(
                self.center.x() - 0.5 * max_size.get(),
                self.center.y() - 0.5 * max_size.get(),
                self.center.z() - 0.5 * max_size.get(),
            ),
            &Point3D::new(
                self.center.x() + 0.5 * max_size.get(),
                self.center.y() + 0.5 * max_size.get(),
                self.center.z() + 0.5 * max_size.get(),
            ),
        )
        .unwrap() //@todo unwrap, see above
    }
}

impl HasBoundingBox3DMaybe for BoxUnaligned3D {
    fn bounding_box_maybe(&self) -> Result<BoundingBox3D> {
        Ok(self.bounding_box())
    }
}

//------------------------------------------------------------------------------

fn rot2d<P>(p: &P, phi: f64) -> P
where
    P: IsBuildable3D,
{
    let s = (-phi).sin();
    let c = (-phi).cos();

    P::new(p.x() * c - p.y() * s, p.x() * s + p.y() * c, p.z())
}

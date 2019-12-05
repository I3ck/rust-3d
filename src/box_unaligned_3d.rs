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

use crate::{functions::*, prelude::*};

//------------------------------------------------------------------------------

/// Not axis aligned Box in 3D space
#[derive(Clone)]
pub struct BoxUnaligned3D {
    //@todo better name (Box3D vs BoxUnaligned3D vs BoundingBox3D)
    pub center: Point3D,
    pub y_dir: [f64; 3], //fwd
    pub z_dir: [f64; 3], //up
    pub size: [Positive; 3],
}

//------------------------------------------------------------------------------

impl BoxUnaligned3D {
    //@todo constructor from the 6 min/max values and one for corners
    pub fn new_from_bb(bb: &BoundingBox3D) -> Self {
        let center = Point3D::new_from(&bb.center_bb());
        let y_dir = [0.0, 1.0, 0.0];
        let z_dir = [0.0, 0.0, 1.0];
        let size = [bb.size_x(), bb.size_y(), bb.size_z()];

        Self {
            center,
            y_dir,
            z_dir,
            size,
        }
    }
    //@todo consider adding alternative constructors
    pub fn new_from_z_rotation<P>(center: &P, size: [Positive; 3], rotation: f64) -> Self
    where
        P: Is3D,
    {
        let mut y_dir = Point3D::new(0.0, 1.0, 0.0);
        let z_dir = [0.0, 0.0, 1.0];

        y_dir = rot2d(&y_dir, rotation);
        Self {
            center: Point3D::new_from(center),
            size,
            y_dir: [y_dir.x(), y_dir.y(), y_dir.z()],
            z_dir,
        }
    }
    fn d_x(&self) -> [f64; 3] {
        //@todo would be easier implemented with proper members
        let dir = self.x_dir();
        let d = self.size[0].get();
        [0.5 * d * dir[0], 0.5 * d * dir[1], 0.5 * d * dir[2]]
    }
    fn d_y(&self) -> [f64; 3] {
        //@todo would be easier implemented with proper members
        let dir = self.y_dir;
        let d = self.size[1].get();
        [0.5 * d * dir[0], 0.5 * d * dir[1], 0.5 * d * dir[2]]
    }
    fn d_z(&self) -> [f64; 3] {
        //@todo would be easier implemented with proper members
        let dir = self.z_dir;
        let d = self.size[2].get();
        [0.5 * d * dir[0], 0.5 * d * dir[1], 0.5 * d * dir[2]]
    }
    pub fn x_dir(&self) -> [f64; 3] {
        let n = Norm3D::new(cross(
            &Point3D::new(self.z_dir[0], self.z_dir[1], self.z_dir[2]),
            &Point3D::new(self.y_dir[0], self.y_dir[1], self.y_dir[2]),
        ))
        .unwrap(); //@todo
        [n.x(), n.y(), n.z()]
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
        let dx = self.d_x();
        let dy = self.d_y();
        let dz = self.d_z();

        //@todo would be better with proper types
        f(&Point3D::new(
            c.x() - dx[0] - dy[0] - dz[0],
            c.y() - dx[1] - dy[1] - dz[1],
            c.z() - dx[2] - dy[2] - dz[2],
        ));

        f(&Point3D::new(
            c.x() - dx[0] - dy[0] + dz[0],
            c.y() - dx[1] - dy[1] + dz[1],
            c.z() - dx[2] - dy[2] + dz[2],
        ));

        f(&Point3D::new(
            c.x() - dx[0] + dy[0] - dz[0],
            c.y() - dx[1] + dy[1] - dz[1],
            c.z() - dx[2] + dy[2] - dz[2],
        ));

        f(&Point3D::new(
            c.x() - dx[0] + dy[0] + dz[0],
            c.y() - dx[1] + dy[1] + dz[1],
            c.z() - dx[2] + dy[2] + dz[2],
        ));

        f(&Point3D::new(
            c.x() + dx[0] - dy[0] - dz[0],
            c.y() + dx[1] - dy[1] - dz[1],
            c.z() + dx[2] - dy[2] - dz[2],
        ));

        f(&Point3D::new(
            c.x() + dx[0] - dy[0] + dz[0],
            c.y() + dx[1] - dy[1] + dz[1],
            c.z() + dx[2] - dy[2] + dz[2],
        ));

        f(&Point3D::new(
            c.x() + dx[0] + dy[0] - dz[0],
            c.y() + dx[1] + dy[1] - dz[1],
            c.z() + dx[2] + dy[2] - dz[2],
        ));

        f(&Point3D::new(
            c.x() + dx[0] + dy[0] + dz[0],
            c.y() + dx[1] + dy[1] + dz[1],
            c.z() + dx[2] + dy[2] + dz[2],
        ));
    }

    fn for_each_axis<F>(&self, f: &mut F)
    where
        F: FnMut(&Norm3D),
    {
        let x_dir = Norm3D::new(Point3D::new(
            self.x_dir()[0],
            self.x_dir()[1],
            self.x_dir()[2],
        ))
        .unwrap(); //@todo hold Norm3D instead //@todo cache x_dir() result
        let y_dir = Norm3D::new(Point3D::new(self.y_dir[0], self.y_dir[1], self.y_dir[2])).unwrap(); //@todo hold Norm3D instead
        let z_dir = Norm3D::new(Point3D::new(self.z_dir[0], self.z_dir[1], self.z_dir[2])).unwrap(); //@todo hold Norm3D instead

        f(&x_dir);
        f(&y_dir);
        f(&z_dir);
    }
}

//------------------------------------------------------------------------------

impl HasBoundingBox3D for BoxUnaligned3D {
    fn bounding_box(&self) -> BoundingBox3D {
        //@todo assumes size is valid, but currently not the case (switch to Positive)
        //@todo not optimal solution
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

//@todo consider moving somewhere else
fn rot2d<P>(p: &P, phi: f64) -> P
where
    P: IsBuildable3D,
{
    let s = (-phi).sin();
    let c = (-phi).cos();

    P::new(p.x() * c - p.y() * s, p.x() * s + p.y() * c, p.z())
}

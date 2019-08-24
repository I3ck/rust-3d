/*
Copyright 2016 Martin Buck

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

//! IsMesh3D trait used for meshes in 3D space

use prelude::*;
use functions::{conn, cross};

/// IsMesh3D is trait used for meshes in 3D space
pub trait IsMesh3D<P> : IsMesh<P, Face3> where
    P: IsBuildable3D {
    /// Returns the normal of a face
    fn face_normal(&self, faceid: FId) -> Result<Norm3D> {
        let [v1, v2, v3] = self.face_vertices(faceid)?;

        let v12 = conn(&v1, &v2);
        let v23 = conn(&v2, &v3);

        let n = cross(&v12, &v23);

        Norm3D::new(n).and_then(|x| Ok(x))
    }
}

impl<M,P> IsMesh3D<P> for M where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D {
}

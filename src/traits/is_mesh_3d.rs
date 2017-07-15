/*
Copyright 2016 Martin Buck
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

//! IsMesh3D trait used for meshes in 3D space

use prelude::*;
use functions::{conn, cross};

/// IsMesh3D is trait used for meshes in 3D space
pub trait IsMesh3D<P> : IsMesh<P> where
    P: IsBuildable3D {
    /// Returns the normal of a face
    fn face_normal(&self, faceid: FId) -> Result<Norm3D> {
        let (v1, v2, v3) = self.face_vertices(faceid)?;

        let v12 = conn(&v1, &v2);
        let v23 = conn(&v2, &v3);

        let n = cross(&v12, &v23);

        Norm3D::new(*n).and_then(|x| Ok(*x))
    }
}

impl<M,P> IsMesh3D<P> for M where
    M: IsMesh<P>,
    P: IsBuildable3D {
}

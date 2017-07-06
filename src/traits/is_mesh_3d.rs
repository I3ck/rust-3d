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

use result::*;
use face3::*;
use traits::is_buildable_3d::*;
use traits::is_normalized_3d::*;
use functions::{conn, cross};
use norm_3d::*;

/// IsMesh3D is trait used for meshes in 3D space
pub trait IsMesh3D<P> where
    P: IsBuildable3D {
    /// Should return the number of faces within the mesh
    fn num_faces(&self) -> usize;
    /// Should return the number of vertices within the mesh
    fn num_vertices(&self) -> usize;
    /// Should return the ids of vertices of the given face
    fn face_vertex_ids(&self, faceid: usize) -> Result<Face3>;
    /// Should return the vertices of the given face
    fn face_vertices(&self, faceid: usize) -> Result<(P, P, P)>;
    /// Should return the vertex with the given id
    fn vertex(&self, vertexid: usize) -> Result<P>;

    /// Returns the normal of a face
    fn face_normal(&self, faceid: usize) -> Result<Norm3D> {
        let (v1, v2, v3) = self.face_vertices(faceid)?;

        let v12 = conn(&v1, &v2);
        let v23 = conn(&v2, &v3);

        let n = cross(&v12, &v23);

        Norm3D::new(*n).and_then(|x| Ok(*x))
    }
}

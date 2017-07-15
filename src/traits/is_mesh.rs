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

//! IsMesh trait used for meshes

use prelude::*;

/// IsMesh is trait used for meshes
pub trait IsMesh<T> {
    /// Should return the number of faces within the mesh
    fn num_faces(&self) -> usize;
    /// Should return the number of vertices within the mesh
    fn num_vertices(&self) -> usize;
    /// Should return the ids of vertices of the given face
    fn face_vertex_ids(&self, faceid: FId) -> Result<Face3>;
    /// Should return the vertices of the given face
    fn face_vertices(&self, faceid: FId) -> Result<(T, T, T)>;
    /// Should return the vertex with the given id
    fn vertex(&self, vertexid: VId) -> Result<T>;
}

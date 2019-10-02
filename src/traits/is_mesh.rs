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

//! IsMesh trait used for meshes

use crate::prelude::*;

/// IsMesh is trait used for meshes
pub trait IsMesh<V, TU> where
    TU: IsTopologyUnit {
    /// Should return the number of faces within the mesh
    fn num_faces(&self) -> usize;
    /// Should return the number of vertices within the mesh
    fn num_vertices(&self) -> usize;
    /// Should return the ids of vertices of the given face
    fn face_vertex_ids(&self, faceid: FId) -> Result<TU>;
    /// Should return the vertices of the given face
    fn face_vertices(&self, faceid: FId) -> Result<[V; 3]>;
    /// Should return the vertex with the given id
    fn vertex(&self, vertexid: VId) -> Result<V>;
}

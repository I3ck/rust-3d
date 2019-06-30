/*
Copyright 2018 Martin Buck

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

//! IsFaceEditableMesh trait used for meshes with editable face data

use prelude::*;

/// IsFaceEditableMesh trait used for meshes with editable face data
pub trait IsFaceEditableMesh<V, TU> : IsMesh<V, TU> where
    TU: IsTopologyUnit {
    /// Should add a face with the 3 positions. Also should return the id of the new face
    fn add_face(&mut self, v1: V, v2: V, v3: V) -> FId;
    /// Should add a face to the mesh by connecting the vertices via their ids. Should return the id of the newly added face
    fn try_add_connection(&mut self, vid1: VId, vid2: VId, vid3: VId) -> Result<FId>;
}

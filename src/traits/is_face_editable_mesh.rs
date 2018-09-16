/*
Copyright 2018 Martin Buck
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

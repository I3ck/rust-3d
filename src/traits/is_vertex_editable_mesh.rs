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

//! IsVertexEditableMesh trait used for meshes with editable vertex data

use prelude::*;

/// IsVertexEditableMesh trait used for meshes with editable vertex data
pub trait IsVertexEditableMesh<V, TU> : IsMesh<V, TU> where
    TU: IsTopologyUnit {
    /// Should add a vertex to the end and return its id
    fn add_vertex(&mut self, vertex: V) -> VId;
    /// Should change vertex at vId to the given vertex returning an error on failure
    fn change_vertex(&mut self, vid: VId, vertex: V) -> Result<()>;
}

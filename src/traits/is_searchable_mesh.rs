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

//! IsSearchableMesh trait used for meshes which have extended search methods

use prelude::*;

/// IsSearchableMesh trait used for meshes which have extended search methods
pub trait IsSearchableMesh<V, TU> : IsMesh<V, Face3> {
    /// Should return the edge ids of the given face. Error if id invalid
    fn edges_of_face(&self, faceid: FId) -> Result<(EId, EId, EId)>;
    /// Should append the edges originating at the given vertex (pointing away / having the vertex as tail). Error if id invalid
    fn edges_originating_from_vertex(&self, vertexid: VId, result: &mut Vec<EId>) -> Result<()>;
    /// Should append the edges ending at the given vertex (pointing to / having the vertex as head). Error if id invalid
    fn edges_ending_at_vertex(&self, vertexid: VId, result: &mut Vec<EId>) -> Result<()>;
    /// Should append the edges connecting with the vertex. Error if id invalid
    fn edges_of_vertex(&self, vertexid: VId, result: &mut Vec<EId>) -> Result<()>;
    /// Should return the vertex id of the edge's tail. Error if id invalid
    fn edge_tail(&self, edgeid: EId) -> Result<VId>;
    /// Should return the vertex id of the edge's head. Error if id invalid
    fn edge_head(&self, edgeid: EId) -> Result<VId>;
    /// Should return the edge id of the next edge. Error if id invalid
    fn edge_next(&self, edgeid: EId) -> Result<EId>;
    /// Should return the edge id of the previous edge. Error if id invalid
    fn edge_prev(&self, edgeid: EId) -> Result<EId>;
    /// Should return the edge id of the twin edge. Error if id invalid, None if there is none
    fn edge_twin(&self, edgeid: EId) -> Result<Option<EId>>;
    /// Should return the face id of the edges face. Error if id invalid
    fn edge_face(&self, edgeid: EId) -> Result<FId>;

    /// Returns the number of edges within the mesh
    fn num_edges(&self) -> usize {
        self.num_faces() * 3
    }
    /// Appends faces a vertex is part of. Error if id invalid
    fn faces_of_vertex(&self, vertexid: VId, result: &mut Vec<FId>) -> Result<()> {
        let mut edgeids = Vec::new(); //@todo try to avoid this
        self.edges_originating_from_vertex(vertexid, &mut edgeids)?;

        for edgeid in edgeids {
            self.edge_face(edgeid).map(|faceid| result.push(faceid))?;
        }
        Ok(())
    }
    /// Appends the neighbouring faces of the given face which share the same edges. Error if id invalid
    fn face_edge_neighbours(&self, faceid: FId, result: &mut Vec<FId>) -> Result<()> {
        let (e1, e2, e3) = self.edges_of_face(faceid)?;

        {
            let mut add_twin_face = |edgeid| self.edge_twin(edgeid).map(|option| match option {
                None => {}
                Some(twin) => { let _ = self.edge_face(twin).map(|x| result.push(x)); }
            });

            add_twin_face(e1)?;
            add_twin_face(e2)?;
            add_twin_face(e3)?;
        }
        Ok(())
    }
    /// Appends the neighbouring faces of the given face which share the same vertices. Sorts and dedups the result. Error if id invalid
    fn face_vertex_neighbours(&self, faceid: FId, result: &mut Vec<FId>) -> Result<()> {
        let vids = self.face_vertex_ids(faceid)?;

        {
            let mut add_vertex_faces = |vertexid| self.faces_of_vertex(vertexid, result);

            add_vertex_faces(vids.a)?;
            add_vertex_faces(vids.b)?;
            add_vertex_faces(vids.c)?;
        }
        result.sort();
        result.dedup();
        Ok(())
    }
}

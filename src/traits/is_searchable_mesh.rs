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

//! IsSearchableMesh trait used for meshes which have extended search methods

use crate::prelude::*;

/// IsSearchableMesh trait used for meshes which have extended search methods
pub trait IsSearchableMesh<V, TU> : IsMesh<V, Face3> {
    /// Should return the edge ids of the given face. Error if id invalid
    fn edges_of_face(&self, faceid: FId) -> Result<(EId, EId, EId)>;
    /// Should append the edges originating at the given vertex (pointing away / having the vertex as tail). Error if id invalid
    fn edges_originating_from_vertex(&self, vertexid: VId, result: &mut Vec<EId>) -> Result<()>;
    /// Should append the edges ending at the given vertex (pointing to / having the vertex as head). Error if id invalid
    /// cache can be any Vec and can be used to store intermediate results avoiding allocations
    fn edges_ending_at_vertex(&self, vertexid: VId, cache: &mut Vec<EId>, result: &mut Vec<EId>) -> Result<()>;
    /// Should append the edges connecting with the vertex. Error if id invalid
    /// cache can be any Vec and can be used to store intermediate results avoiding allocations
    fn edges_of_vertex(&self, vertexid: VId, cache: &mut Vec<EId>, result: &mut Vec<EId>) -> Result<()>;
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
    /// cache can be any Vec
    fn faces_of_vertex(&self, vertexid: VId, cache: &mut Vec<EId>, result: &mut Vec<FId>) -> Result<()> {
        cache.clear();
        self.edges_originating_from_vertex(vertexid, cache)?;

        for edgeid in cache {
            self.edge_face(*edgeid).map(|faceid| result.push(faceid))?;
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
    /// cache can be any Vec
    fn face_vertex_neighbours(&self, faceid: FId, cache: &mut Vec<EId>, result: &mut Vec<FId>) -> Result<()> {
        cache.clear();
        let vids = self.face_vertex_ids(faceid)?;

        {
            let mut add_vertex_faces = |vertexid| self.faces_of_vertex(vertexid, cache, result);

            add_vertex_faces(vids.a)?;
            add_vertex_faces(vids.b)?;
            add_vertex_faces(vids.c)?;
        }
        result.sort();
        result.dedup();
        Ok(())
    }
}

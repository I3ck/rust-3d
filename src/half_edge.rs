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

//! HalfEdge, the half edge data structure

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// HalfEdge, the half edge data structure
pub struct HalfEdge<IC>
where
    IC: IsIndexContainer,
{
    tails: IC,
    twins: Vec<Option<EId>>,
    vertices_start_edges: Vec<IC>,
}

impl<IC> HalfEdge<IC>
where
    IC: IsIndexContainer,
{
    /// Creates a new HalfEdge3D for the given IsMesh3D
    /// This only stays valid if IMesh3D is not changed after creation
    /// The mesh must be manifold
    pub fn new<T, M>(mesh: &M) -> Self
    where
        M: IsMesh<T, Face3>,
    {
        let n_vertices = mesh.num_vertices();
        let n_faces = mesh.num_faces();
        let n_edges = 3 * n_faces;

        let mut tails = IC::with_capacity_and_support_for(n_edges, n_vertices);

        let twins = vec![None; 3 * n_faces];
        let mut vertices_start_edges = Vec::with_capacity(n_vertices);

        for i in 0..n_faces {
            match mesh.face_vertex_ids(FId { val: i }) {
                Err(_) => {}
                Ok(face) => {
                    tails.push(face.a.val);
                    tails.push(face.b.val);
                    tails.push(face.c.val);

                    safe_append_at(&mut vertices_start_edges, n_edges, face.a.val, i * 3 + 0);
                    safe_append_at(&mut vertices_start_edges, n_edges, face.b.val, i * 3 + 1);
                    safe_append_at(&mut vertices_start_edges, n_edges, face.c.val, i * 3 + 2);
                }
            }
        }

        let mut result = HalfEdge {
            tails,
            twins,
            vertices_start_edges: vertices_start_edges,
        };

        // For each edge, get tail of next
        // Of this get all edges originating
        // Of these the one where next has the same tail must be the twin
        // @todo could let this fail if there is more than one valid candidate (not manifold)
        let mut cache = Vec::new();
        for i in 0..result.tails.len() {
            cache.clear();
            let _ = result.next(EId { val: i }).and_then(&mut |next_id: EId| {
                result.edges_originating(
                    VId {
                        val: result.tails.get(next_id.val),
                    },
                    &mut cache,
                )
            });
            for originating_id in cache.iter() {
                let _ = result.next(*originating_id).map(|candidate_id| {
                    if result.tails.get(candidate_id.val) == result.tails.get(i) {
                        result.twins[i] = Some(candidate_id)
                    }
                });
            }
        }
        result
    }
    /// Returns the ID of the vertex the edge originates from (error if id out of bounds)
    pub fn tail(&self, id: EId) -> Result<VId> {
        self.ensure_edge_id(id)?;
        Ok(VId {
            val: self.tails.get(id.val),
        })
    }
    /// Returns the ID of the face the edge belongs to (error if id out of bounds)
    pub fn face(&self, id: EId) -> Result<FId> {
        self.ensure_edge_id(id)?;
        Ok(FId { val: id.val / 3 })
    }
    /// Returns the ID of the twin edge (None if there isn't any) (error if id out of bounds)
    pub fn twin(&self, id: EId) -> Result<Option<EId>> {
        self.ensure_edge_id(id)?;
        Ok(self.twins[id.val])
    }
    /// Returns the ID of the edge after this edge (error if id out of bounds)
    pub fn next(&self, id: EId) -> Result<EId> {
        self.ensure_edge_id(id)?;
        if Self::last_in_face(id) {
            return Ok(EId { val: id.val - 2 });
        }
        Ok(EId { val: id.val + 1 })
    }
    /// Returns the ID of the edge before this edge (error if id out of bounds)
    pub fn prev(&self, id: EId) -> Result<EId> {
        self.ensure_edge_id(id)?;
        if Self::first_in_face(id) {
            return Ok(EId { val: id.val + 2 });
        }
        Ok(EId { val: id.val - 1 })
    }
    /// Appends all edges originating (pointing away) from the given vertex (error if id out of bounds)
    pub fn edges_originating(&self, id: VId, result: &mut Vec<EId>) -> Result<()> {
        self.ensure_vertex_id(id)?;
        result.extend(
            self.vertices_start_edges[id.val]
                .iter()
                .map(|x| EId { val: x }),
        );
        Ok(())
    }
    /// Appends all edges ending (pointing at) the given vertex (error if id out of bounds)
    /// cache is used to avoid allocations, pass any Vec
    pub fn edges_ending(&self, id: VId, cache: &mut Vec<EId>, result: &mut Vec<EId>) -> Result<()> {
        cache.clear();
        self.edges_originating(id, cache)?;
        for edge in cache {
            match self.prev(*edge) {
                Err(_) => {}
                Ok(id) => result.push(id),
            }
        }
        Ok(())
    }
    /// Appends all edges connected to the vertex (both originating and ending) (error if id out of bounds)
    /// cache is used to avoid allocations, pass any Vec
    pub fn edges_all(&self, id: VId, cache: &mut Vec<EId>, result: &mut Vec<EId>) -> Result<()> {
        cache.clear();
        self.edges_originating(id, cache)?;
        for edge in cache {
            result.push(*edge);
            match self.prev(*edge) {
                Err(_) => {}
                Ok(id) => result.push(id),
            }
        }
        Ok(())
    }
    /// Appends all faces a vertex is part of (error if id out of bounds)
    /// cache is used to avoid allocations, pass any Vec
    pub fn faces(&self, id: VId, cache: &mut Vec<EId>, result: &mut Vec<FId>) -> Result<()> {
        cache.clear();
        self.edges_originating(id, cache)?;
        for edge in cache {
            match self.face(*edge) {
                Err(_) => {}
                Ok(id) => result.push(id),
            }
        }
        Ok(())
    }
    /// Returns true if the give edge is the first within a face
    fn first_in_face(id: EId) -> bool {
        id.val % 3 == 0
    }
    /// Returns true if the give edge is the last within a face
    fn last_in_face(id: EId) -> bool {
        id.val % 3 == 2
    }
    /// Fails if the edge ID is out of bounds
    pub fn ensure_edge_id(&self, id: EId) -> Result<()> {
        if id.val >= self.tails.len() {
            return Err(ErrorKind::IncorrectEdgeID);
        }
        Ok(())
    }
    /// Fails if the vertex ID is out of bounds
    pub fn ensure_vertex_id(&self, id: VId) -> Result<()> {
        if id.val >= self.vertices_start_edges.len() {
            return Err(ErrorKind::IncorrectVertexID);
        }
        Ok(())
    }
}

impl<IC> From<(IC, Vec<Option<EId>>, Vec<IC>)> for HalfEdge<IC>
where
    IC: IsIndexContainer,
{
    fn from(ev: (IC, Vec<Option<EId>>, Vec<IC>)) -> Self {
        Self {
            tails: ev.0,
            twins: ev.1,
            vertices_start_edges: ev.2,
        }
    }
}

impl<IC> Into<(IC, Vec<Option<EId>>, Vec<IC>)> for HalfEdge<IC>
where
    IC: IsIndexContainer,
{
    fn into(self) -> (IC, Vec<Option<EId>>, Vec<IC>) {
        (self.tails, self.twins, self.vertices_start_edges)
    }
}

//@todo consider two separate implementations
impl<IC> Into<(IC, Vec<Option<EId>>)> for HalfEdge<IC>
where
    IC: IsIndexContainer,
{
    fn into(self) -> (IC, Vec<Option<EId>>) {
        (self.tails, self.twins)
    }
}

impl<IC> Into<Vec<IC>> for HalfEdge<IC>
where
    IC: IsIndexContainer,
{
    fn into(self) -> Vec<IC> {
        self.vertices_start_edges
    }
}

//------------------------------------------------------------------------------

fn safe_append_at<IC>(vec: &mut Vec<IC>, max: usize, i: usize, val: usize)
where
    IC: IsIndexContainer,
{
    if i >= vec.len() {
        vec.resize(i + 1, IC::with_support_for(max));
    }

    vec[i].push(val);
}

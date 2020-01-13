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

//! SearchableMesh, transforms IsMesh to IsSearchableMesh

use crate::*;

use std::marker::PhantomData;

/// SearchableMesh, transforms IsMesh to IsSearchableMesh
#[derive(Clone)]
pub struct SearchableMesh<M, T>
where
    M: IsMesh<T, Face3>,
{
    mesh: M,
    he: HalfEdge,
    phantomt: PhantomData<T>,
}

impl<M, T> SearchableMesh<M, T>
where
    M: IsMesh<T, Face3>,
{
    /// Creates a new SearchableMesh3D from an IsMesh3D
    /// This only stays valid if IMesh3D is not changed after creation
    /// The mesh must be manifold
    pub fn new(mesh: M) -> Self {
        let he = HalfEdge::new(&mesh);

        SearchableMesh {
            mesh,
            he,
            phantomt: PhantomData,
        }
    }

    /// Fails if the vertex ID is out of bounds
    pub fn ensure_face_id(&self, id: FId) -> Result<()> {
        if id.val >= self.mesh.num_faces() {
            return Err(ErrorKind::IncorrectFaceID);
        }
        Ok(())
    }

    /// Returns a reference to the held mesh
    pub fn mesh(&self) -> &M {
        &self.mesh
    }
}

impl<M, T> IsMesh<T, Face3> for SearchableMesh<M, T>
where
    M: IsMesh<T, Face3>,
{
    fn num_faces(&self) -> usize {
        self.mesh.num_faces()
    }

    fn num_vertices(&self) -> usize {
        self.mesh.num_vertices()
    }

    fn face_vertex_ids(&self, faceid: FId) -> Result<Face3> {
        self.mesh.face_vertex_ids(faceid)
    }

    fn face_vertices(&self, faceid: FId) -> Result<[T; 3]> {
        self.mesh.face_vertices(faceid)
    }

    fn vertex(&self, vertexid: VId) -> Result<T> {
        self.mesh.vertex(vertexid)
    }
}

impl<M, T> IsVertexEditableMesh<T, Face3> for SearchableMesh<M, T>
where
    M: IsMesh<T, Face3> + IsVertexEditableMesh<T, Face3>,
    T: IsEditable3D + IsBuildable3D + Clone,
{
    fn add_vertex(&mut self, vertex: T) -> VId {
        self.mesh.add_vertex(vertex)
    }

    fn change_vertex(&mut self, vid: VId, vertex: T) -> Result<()> {
        self.mesh.change_vertex(vid, vertex)
    }
}

impl<M, T> IsSearchableMesh<T, Face3> for SearchableMesh<M, T>
where
    M: IsMesh<T, Face3>,
{
    fn num_edges(&self) -> usize {
        self.mesh.num_faces() * 3
    }

    fn edges_of_face(&self, faceid: FId) -> Result<(EId, EId, EId)> {
        self.ensure_face_id(faceid)?;
        Ok((
            EId {
                val: faceid.val * 3 + 0,
            },
            EId {
                val: faceid.val * 3 + 1,
            },
            EId {
                val: faceid.val * 3 + 2,
            },
        ))
    }

    fn edges_originating_from_vertex(&self, vertexid: VId, result: &mut Vec<EId>) -> Result<()> {
        self.he.edges_originating(vertexid, result)
    }

    fn edges_ending_at_vertex(
        &self,
        vertexid: VId,
        cache: &mut Vec<EId>,
        result: &mut Vec<EId>,
    ) -> Result<()> {
        self.he.edges_ending(vertexid, cache, result)
    }

    fn edges_of_vertex(
        &self,
        vertexid: VId,
        cache: &mut Vec<EId>,
        result: &mut Vec<EId>,
    ) -> Result<()> {
        self.he.edges_all(vertexid, cache, result)
    }

    fn edge_tail(&self, edgeid: EId) -> Result<VId> {
        self.he.tail(edgeid)
    }

    fn edge_head(&self, edgeid: EId) -> Result<VId> {
        self.he.next(edgeid).and_then(|next| self.he.tail(next))
    }

    fn edge_next(&self, edgeid: EId) -> Result<EId> {
        self.he.next(edgeid)
    }

    fn edge_prev(&self, edgeid: EId) -> Result<EId> {
        self.he.prev(edgeid)
    }

    fn edge_twin(&self, edgeid: EId) -> Result<Option<EId>> {
        self.he.twin(edgeid)
    }

    fn edge_face(&self, edgeid: EId) -> Result<FId> {
        self.he.face(edgeid)
    }
}

impl<M, T> HasBoundingBox3DMaybe for SearchableMesh<M, T>
where
    M: IsMesh<T, Face3> + HasBoundingBox3DMaybe,
{
    fn bounding_box_maybe(&self) -> Result<BoundingBox3D> {
        self.mesh.bounding_box_maybe()
    }
}

impl<M, T> HasCenterOfGravity3D for SearchableMesh<M, T>
where
    M: IsMesh<T, Face3> + HasCenterOfGravity3D,
{
    fn center_of_gravity(&self) -> Result<Point3D> {
        self.mesh.center_of_gravity()
    }
}

impl<M, T> IsScalable for SearchableMesh<M, T>
where
    M: IsMesh<T, Face3> + IsScalable,
{
    fn scale(&mut self, factor: Positive) {
        self.mesh.scale(factor);
    }
}

impl<M, T> AsRef<M> for SearchableMesh<M, T>
where
    M: IsMesh<T, Face3>,
{
    fn as_ref(&self) -> &M {
        &self.mesh
    }
}

impl<M, T> IsDestructible<(M, HalfEdge)> for SearchableMesh<M, T>
where
    M: IsMesh<T, Face3>,
{
    fn destructed(self) -> (M, HalfEdge) {
        (self.mesh, self.he)
    }
}

impl<M, T> IsDestructible<M> for SearchableMesh<M, T>
where
    M: IsMesh<T, Face3>,
{
    fn destructed(self) -> M {
        self.mesh
    }
}

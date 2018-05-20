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

//! SearchableMesh, transforms IsMesh to IsSearchableMesh

use prelude::*;

use std::marker::PhantomData;

/// SearchableMesh, transforms IsMesh to IsSearchableMesh
pub struct SearchableMesh<M, T> where 
    M: IsMesh<T, Face3> {
    
    mesh: M,
    he: HalfEdge,
    phantomt: PhantomData<T>
}

impl<M, T> SearchableMesh<M, T> where
    M: IsMesh<T, Face3> {
    /// Creates a new SearchableMesh3D from an IsMesh3D
    /// This only stays valid if IMesh3D is not changed after creation
    /// The mesh must be manifold (@todo ensure via types?)
    pub fn new(mesh: M) -> Self {

        let he = HalfEdge::new(&mesh);

        SearchableMesh {mesh, he, phantomt: PhantomData}
    }

    /// Fails if the vertex ID is out of bounds
    pub fn ensure_face_id(&self, id: FId) -> Result<()> {
        if id.val >= self.mesh.num_faces() {
            return Err(ErrorKind::IncorrectFaceID);
        }
        Ok(())
    }
}

impl<M, T> IsMesh<T, Face3> for SearchableMesh<M, T> where
    M: IsMesh<T, Face3> {
    
    fn num_faces(&self) -> usize {
        self.mesh.num_faces()
    }

    fn num_vertices(&self) -> usize {
        self.mesh.num_vertices()
    }

    fn face_vertex_ids(&self, faceid: FId) -> Result<Face3> {
        self.mesh.face_vertex_ids(faceid)
    }

    fn face_vertices(&self, faceid: FId) -> Result<(T, T, T)> {
        self.mesh.face_vertices(faceid)
    }

    fn vertex(&self, vertexid: VId) -> Result<T> {
        self.mesh.vertex(vertexid)
    }
}

impl<M, T> IsSearchableMesh<T, Face3> for SearchableMesh<M, T> where
    M: IsMesh<T, Face3> {

    fn num_edges(&self) -> usize {
        self.mesh.num_faces() * 3
    }

    fn edges_of_face(&self, faceid: FId) -> Result<(EId, EId, EId)> {
        self.ensure_face_id(faceid)?;
        Ok((EId{val: faceid.val * 3 + 0},
            EId{val: faceid.val * 3 + 1},
            EId{val: faceid.val * 3 + 2}))
    }

    fn edges_originating_from_vertex(&self, vertexid: VId) -> Result<Vec<EId>> {
        self.he.edges_originating(vertexid)
    }

    fn edges_ending_at_vertex(&self, vertexid: VId) -> Result<Vec<EId>> {
        self.he.edges_ending(vertexid)
    }

    fn edges_of_vertex(&self, vertexid: VId) -> Result<Vec<EId>> {
        self.he.edges_all(vertexid)
    }

    fn edge_tail(&self, edgeid: EId) -> Result<VId> {
        self.he.tail(edgeid)
    }

    fn edge_head(&self, edgeid: EId) -> Result<VId> {
        self.he.next(edgeid)
            .and_then(|next| self.he.tail(next))
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

impl<M, T> HasBoundingBox3D for SearchableMesh<M, T> where
    M: IsMesh<T, Face3> + HasBoundingBox3D {

    fn bounding_box(&self) -> Result<BoundingBox3D> {
        self.mesh.bounding_box()
    }
}

impl<M, T> HasCenterOfGravity3D for SearchableMesh<M, T> where
    M: IsMesh<T, Face3> + HasCenterOfGravity3D {

    fn center_of_gravity(&self) -> Result<Point3D> {
        self.mesh.center_of_gravity()
    }
}

impl<M, T> IsScalable for SearchableMesh<M, T> where
    M: IsMesh<T, Face3> + IsScalable {
    
    fn scale(&mut self, factor: Positive) {
        self.mesh.scale(factor);
    }
}
    



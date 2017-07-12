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

//! IsSearchableMesh3D trait used for meshes in 3D space which have extended search methods

use prelude::*;

/// IsSearchableMesh3D trait used for meshes in 3D space which have extended search methods
pub trait IsSearchableMesh3D<P> : IsMesh3D<P> where
    P: IsBuildable3D {

    fn num_edges(&self) -> usize;

    fn edges_of_face(&self, faceid: FId) -> Result<(EId, EId, EId)>;

    fn edges_originating_from_vertex(&self, vertexid: VId) -> Result<Vec<EId>>;
    fn edges_ending_at_vertex       (&self, vertexid: VId) -> Result<Vec<EId>>;
    fn edges_of_vertex              (&self, vertexid: VId) -> Result<Vec<EId>>;

    fn edge_tail(&self, edgeid: EId) -> Result<VId>;
    fn edge_head(&self, edgeid: EId) -> Result<VId>;
    fn edge_next(&self, edgeid: EId) -> Result<EId>;
    fn edge_prev(&self, edgeid: EId) -> Result<EId>;
    fn edge_twin(&self, edgeid: EId) -> Result<Option<EId>>;
    fn edge_face(&self, edgeid: EId) -> Result<FId>;


    fn faces_of_vertex(&self, vertexid: VId) -> Result<Vec<FId>> {
        let edgeids = self.edges_originating_from_vertex(vertexid)?;

        let mut result = Vec::with_capacity(edgeids.len());
        for edgeid in edgeids {
            self.edge_face(edgeid).map(|faceid| result.push(faceid))?;
        }
        Ok(result)
    }

    fn face_edge_neighbours(&self, faceid: FId) -> Result<Vec<FId>> {
        let (e1, e2, e3) = self.edges_of_face(faceid)?;

        let mut result = Vec::new();
        {
            let mut add_twin_face = |edgeid| self.edge_twin(edgeid).map(|option| match option {
                None => {}
                Some(twin) => { let _ = self.edge_face(twin).map(|x| result.push(x)); }
            });

            add_twin_face(e1)?;
            add_twin_face(e2)?;
            add_twin_face(e3)?;
        }

        Ok(result)
    }

    fn face_vertex_neighbours(&self, faceid: FId) -> Result<Vec<FId>> {
        let vids = self.face_vertex_ids(faceid)?;

        let mut result = Vec::new();
        {
            let mut add_vertex_faces = |vertexid| self.faces_of_vertex(vertexid)
                .map(|fids| for fid in fids {
                    result.push(fid);
                });

            add_vertex_faces(vids.a)?;
            add_vertex_faces(vids.b)?;
            add_vertex_faces(vids.c)?;
        }
        result.sort();
        result.dedup();
        Ok(result)
    }
}

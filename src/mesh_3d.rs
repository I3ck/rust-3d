/*
Copyright 2016 Martin Buck
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

//! Mesh3D, a mesh with tri-faces within 3D space

use prelude::*;

#[derive (Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// Mesh3D, a mesh with tri-faces within 3D space
pub struct Mesh3D<P> where
    P: Is3D {

    pc: PointCloud3D<P>,
    topology: Vec<VId>
}

impl<P> IsMesh<P, Face3> for Mesh3D<P> where
    P: IsBuildable3D + Clone {

    fn num_faces(&self) -> usize {
        self.topology.len() / 3
    }

    fn num_vertices(&self) -> usize {
        self.pc.len()
    }

    fn face_vertex_ids(&self, faceid: FId) -> Result<Face3> {
        let id1 = 3*faceid.val + 0;
        let id2 = 3*faceid.val + 1;
        let id3 = 3*faceid.val + 2;

        if id3 >= self.topology.len() {
            return Err(ErrorKind::IncorrectFaceID);
        }

        Ok(Face3::new(self.topology[id1], self.topology[id2], self.topology[id3]))
    }

    fn face_vertices(&self, faceid: FId) -> Result<(P, P, P)> {
        let face = self.face_vertex_ids(faceid)?;
        if let (Ok(v1), Ok(v2), Ok(v3)) = (self.vertex(face.a), self.vertex(face.b), self.vertex(face.c)) {
            return Ok((v1, v2, v3));
        }
        Err(ErrorKind::IncorrectVertexID)
    }

    fn vertex(&self, vertexid: VId) -> Result<P> {
        if vertexid.val >= self.pc.len() {
            return Err(ErrorKind::IncorrectVertexID);
        }
        Ok(*self.pc.data[vertexid.val].clone())
    }
}

impl<P> IsEditableMesh<P, Face3> for Mesh3D<P> where
    P: IsEditable3D + IsBuildable3D + Clone {

    fn add_vertex(&mut self, vertex: P) -> VId {
        self.pc.push(vertex);
        VId{val: self.pc.len() - 1}
    }

    fn add_face(&mut self, v1: P, v2: P, v3: P) -> FId {
        let vid1 = self.add_vertex(v1);
        let vid2 = self.add_vertex(v2);
        let vid3 = self.add_vertex(v3);
        self.topology.push(vid1);
        self.topology.push(vid2);
        self.topology.push(vid3);
        FId{val: self.topology.len() / 3 - 1}
    }

    fn try_add_connection(&mut self, vid1: VId, vid2: VId, vid3: VId) -> Result<FId> {
        if vid1.val >= self.pc.len() || vid2.val >= self.pc.len() || vid3.val >= self.pc.len() || vid1 == vid2 || vid1 == vid3 || vid2 == vid3 {
            return Err(ErrorKind::IncorrectVertexID);
        }
        self.topology.push(vid1);
        self.topology.push(vid2);
        self.topology.push(vid3);
        Ok(FId{val: self.topology.len() / 3 - 1})
    }
}

impl<P> HasBoundingBox3D for Mesh3D<P> where
    P: Is3D {

    fn bounding_box(&self) -> Result<BoundingBox3D> {
        self.pc.bounding_box()
    }
}

impl<P> HasCenterOfGravity3D for Mesh3D<P> where
    P: Is3D {

    fn center_of_gravity(&self) -> Result<Point3D> {
        self.pc.center_of_gravity()
    }
}

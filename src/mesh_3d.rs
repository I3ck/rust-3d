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

use result::*;
use traits::is_mesh_3d::IsMesh3D;
use traits::is_editable_mesh_3d::IsEditableMesh3D;
use traits::is_3d::Is3D;
use traits::is_editable_3d::IsEditable3D;
use traits::is_buildable_3d::IsBuildable3D;
use traits::has_bounding_box_3d::*;
use point_3d::*;
use point_cloud_3d::PointCloud3D;

pub struct Mesh3D<P> where
    P: Is3D {

    pub pc: PointCloud3D<P>,
    pub topology: Vec<usize>
}

impl<P> IsMesh3D<P> for Mesh3D<P> where
    P: IsBuildable3D + Clone {

    fn num_faces(&self) -> usize {
        self.topology.len() / 3
    }

    fn num_vertices(&self) -> usize {
        self.pc.len()
    }

    fn face_vertex_ids(&self, faceid: usize) -> Result<(usize, usize, usize)> {
        let id1 = 3*faceid + 0;
        let id2 = 3*faceid + 1;
        let id3 = 3*faceid + 2;

        if id3 >= self.topology.len() {
            return Err(ErrorKind::IncorrectFaceID);
        }

        Ok((self.topology[id1], self.topology[id2], self.topology[id3]))
    }

    fn face_vertices(&self, faceid: usize) -> Result<(P, P, P)> {
        let (id1, id2, id3) = try!(self.face_vertex_ids(faceid));
        if let (Ok(v1), Ok(v2), Ok(v3)) = (self.vertex(id1), self.vertex(id2), self.vertex(id3)) {
            return Ok((v1, v2, v3));
        }
        Err(ErrorKind::IncorrectVertexID)
    }

    fn vertex(&self, vertexid: usize) -> Result<P> {
        if vertexid >= self.pc.len() {
            return Err(ErrorKind::IncorrectVertexID);
        }
        Ok(*self.pc.data[vertexid].clone())
    }
}

impl<P> IsEditableMesh3D<P> for Mesh3D<P> where
    P: IsEditable3D + IsBuildable3D + Clone {

    fn new() -> Self {
        Mesh3D {
            pc: PointCloud3D::new(),
            topology: Vec::new()
        }
    }

    fn add_vertex(&mut self, vertex: P) -> usize {
        self.pc.push(vertex);
        self.pc.len() - 1
    }

    fn add_face(&mut self, v1: P, v2: P, v3: P) -> usize {
        let vid1 = self.add_vertex(v1);
        let vid2 = self.add_vertex(v2);
        let vid3 = self.add_vertex(v3);
        self.topology.push(vid1);
        self.topology.push(vid2);
        self.topology.push(vid3);
        self.topology.len() / 3 - 1
    }

    fn try_add_connection(&mut self, vid1: usize, vid2: usize, vid3: usize) -> Result<usize> {
        if vid1 >= self.pc.len() || vid2 >= self.pc.len() || vid3 >= self.pc.len() || vid1 == vid2 || vid1 == vid3 || vid2 == vid3 {
            return Err(ErrorKind::IncorrectVertexID);
        }
        self.topology.push(vid1);
        self.topology.push(vid2);
        self.topology.push(vid3);
        Ok(self.topology.len() / 3 - 1)
    }
}

impl<P> HasBoundingBox3D for Mesh3D<P> where
    P: Is3D {

    fn bounding_box(&self) -> Result<(Point3D, Point3D)> {
        self.pc.bounding_box()
    }

}

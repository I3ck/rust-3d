/*
Copyright 2016 Martin Buck

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

//! Mesh3D, a mesh with tri-faces within 3D space

use crate::*;

#[derive(Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// Mesh3D, a mesh with tri-faces within 3D space
pub struct Mesh3D<P, IC>
where
    P: Is3D,
    IC: IsIndexContainer + Clone + Default,
{
    pc: PointCloud3D<P>,
    topology: IC,
}

impl<P, IC> Mesh3D<P, IC>
where
    P: Is3D,
    IC: IsIndexContainer + Clone + Default,
{
    /// Reserves number of vertices
    pub fn reserve_vertices(&mut self, n: usize) {
        self.pc.reserve_vertices(n)
    }
    /// Reserves number of faces
    pub fn reserve_faces(&mut self, n: usize) {
        self.topology.reserve(3 * n)
    }
}

impl<P, IC> IsMesh<P, Face3> for Mesh3D<P, IC>
where
    P: Is3D + Clone,
    IC: IsIndexContainer + Clone + Default,
{
    fn num_faces(&self) -> usize {
        self.topology.len() / 3
    }

    fn num_vertices(&self) -> usize {
        self.pc.len()
    }

    fn face_vertex_ids(&self, faceid: FId) -> Result<Face3> {
        let id1 = 3 * faceid.val + 0;
        let id2 = 3 * faceid.val + 1;
        let id3 = 3 * faceid.val + 2;

        if id3 >= self.topology.len() {
            return Err(ErrorKind::IncorrectFaceID);
        }

        Ok(Face3::new(
            VId {
                val: self.topology.get(id1),
            },
            VId {
                val: self.topology.get(id2),
            },
            VId {
                val: self.topology.get(id3),
            },
        ))
    }

    fn face_vertices(&self, faceid: FId) -> Result<[P; 3]> {
        let face = self.face_vertex_ids(faceid)?;
        if let (Ok(v1), Ok(v2), Ok(v3)) = (
            self.vertex(face.a),
            self.vertex(face.b),
            self.vertex(face.c),
        ) {
            return Ok([v1, v2, v3]);
        }
        Err(ErrorKind::IncorrectVertexID)
    }

    fn vertex(&self, vertexid: VId) -> Result<P> {
        if vertexid.val >= self.pc.len() {
            return Err(ErrorKind::IncorrectVertexID);
        }
        Ok(self.pc.data[vertexid.val].clone())
    }
}

impl<P, IC> IsFaceEditableMesh<P, Face3> for Mesh3D<P, IC>
where
    P: IsEditable3D + IsBuildable3D + Clone,
    IC: IsIndexContainer + Clone + Default,
{
    fn add_face(&mut self, v1: P, v2: P, v3: P) -> FId {
        let vid1 = self.add_vertex(v1);
        let vid2 = self.add_vertex(v2);
        let vid3 = self.add_vertex(v3);
        self.topology.push(vid1.val);
        self.topology.push(vid2.val);
        self.topology.push(vid3.val);
        FId {
            val: self.topology.len() / 3 - 1,
        }
    }

    fn try_add_connection(&mut self, vid1: VId, vid2: VId, vid3: VId) -> Result<FId> {
        if vid1.val >= self.pc.len()
            || vid2.val >= self.pc.len()
            || vid3.val >= self.pc.len()
            || vid1 == vid2
            || vid1 == vid3
            || vid2 == vid3
        {
            return Err(ErrorKind::IncorrectVertexID);
        }
        self.topology.push(vid1.val);
        self.topology.push(vid2.val);
        self.topology.push(vid3.val);
        Ok(FId {
            val: self.topology.len() / 3 - 1,
        })
    }
}

impl<P, IC> IsVertexEditableMesh<P, Face3> for Mesh3D<P, IC>
where
    P: IsEditable3D + IsBuildable3D + Clone,
    IC: IsIndexContainer + Clone + Default,
{
    fn add_vertex(&mut self, vertex: P) -> VId {
        self.pc.push(vertex);
        VId {
            val: self.pc.len() - 1,
        }
    }

    fn change_vertex(&mut self, vid: VId, vertex: P) -> Result<()> {
        if vid.val < self.pc.len() {
            self.pc[vid.val] = vertex;
            Ok(())
        } else {
            Err(ErrorKind::IncorrectVertexID)
        }
    }
}

impl<P, IC> HasBoundingBox3DMaybe for Mesh3D<P, IC>
where
    P: Is3D,
    IC: IsIndexContainer + Clone + Default,
{
    fn bounding_box_maybe(&self) -> Result<BoundingBox3D> {
        self.pc.bounding_box_maybe()
    }
}

impl<P, IC> HasCenterOfGravity3D for Mesh3D<P, IC>
where
    P: Is3D,
    IC: IsIndexContainer + Clone + Default,
{
    fn center_of_gravity(&self) -> Result<Point3D> {
        self.pc.center_of_gravity()
    }
}

impl<P, IC> IsScalable for Mesh3D<P, IC>
where
    P: IsEditable3D,
    IC: IsIndexContainer + Clone + Default,
{
    fn scale(&mut self, factor: Positive) {
        self.pc.scale(factor);
    }
}

impl<P, IC> IsMatrix4Transformable for Mesh3D<P, IC>
where
    P: Is3D + IsMatrix4Transformable + Clone,
    IC: IsIndexContainer + Clone + Default,
{
    fn transformed(&self, m: &Matrix4) -> Self {
        let mut new = self.clone();
        new.transform(m);
        new
    }

    fn transform(&mut self, m: &Matrix4) {
        self.pc.transform(m);
    }
}

impl<P, IC> IsMovable3D for Mesh3D<P, IC>
where
    P: Is3D + IsMovable3D,
    IC: IsIndexContainer + Clone + Default,
{
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.pc.move_by(x, y, z)
    }
}

impl<P, IC> From<(PointCloud3D<P>, IC)> for Mesh3D<P, IC>
where
    P: Is3D,
    IC: IsIndexContainer + Clone + Default,
{
    fn from(pt: (PointCloud3D<P>, IC)) -> Self {
        Self {
            pc: pt.0,
            topology: pt.1,
        }
    }
}

impl<P, IC> Into<(PointCloud3D<P>, IC)> for Mesh3D<P, IC>
where
    P: Is3D,
    IC: IsIndexContainer + Clone + Default,
{
    fn into(self) -> (PointCloud3D<P>, IC) {
        (self.pc, self.topology)
    }
}

impl<P, IC> Into<PointCloud3D<P>> for Mesh3D<P, IC>
where
    P: Is3D,
    IC: IsIndexContainer + Clone + Default,
{
    fn into(self) -> PointCloud3D<P> {
        self.pc
    }
}

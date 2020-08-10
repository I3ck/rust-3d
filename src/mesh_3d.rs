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

use std::marker::PhantomData;

//------------------------------------------------------------------------------

#[derive(Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// Mesh3D, a mesh with tri-faces within 3D space
pub struct Mesh3D<P, ID, IC>
where
    P: Is3D,
    ID: IsDataContainer<P>,
    IC: IsIndexContainer,
{
    pc: ID,
    topology: IC,
    _phantom: PhantomData<P>,
}

impl<P, ID, IC> IsMesh<P, Face3> for Mesh3D<P, ID, IC>
where
    P: Is3D + Clone,
    ID: IsDataContainer<P>,
    IC: IsIndexContainer,
{
    fn num_faces(&self) -> usize {
        self.topology.len() / 3
    }

    fn num_vertices(&self) -> usize {
        self.pc.len_d()
    }

    fn face_vertex_ids(&self, faceid: FId) -> Option<Face3> {
        let id1 = 3 * faceid.val + 0;
        let id2 = 3 * faceid.val + 1;
        let id3 = 3 * faceid.val + 2;

        if id3 >= self.topology.len() {
            return None;
        }

        Some(Face3::new(
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

    fn face_vertices(&self, faceid: FId) -> Option<[P; 3]> {
        let face = self.face_vertex_ids(faceid)?;

        let v1 = self.vertex(face.a)?;
        let v2 = self.vertex(face.b)?;
        let v3 = self.vertex(face.c)?;
        Some([v1, v2, v3])
    }

    fn vertex(&self, vertexid: VId) -> Option<P> {
        self.pc.get_d(vertexid.val)
    }
}

impl<P, ID, IC> IsFaceEditableMesh<P, Face3> for Mesh3D<P, ID, IC>
where
    P: IsEditable3D + IsBuildable3D + Clone,
    ID: IsDataContainer<P>,
    IC: IsIndexContainer,
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
        if vid1.val >= self.pc.len_d() || vid2.val >= self.pc.len_d() || vid3.val >= self.pc.len_d()
        {
            return Err(ErrorKind::IncorrectVertexID);
        }
        if vid1 == vid2 || vid1 == vid3 || vid2 == vid3 {
            return Err(ErrorKind::FaceIDsNotUnique);
        }
        self.topology.push(vid1.val);
        self.topology.push(vid2.val);
        self.topology.push(vid3.val);
        Ok(FId {
            val: self.topology.len() / 3 - 1,
        })
    }

    fn reserve_faces(&mut self, n: usize) {
        self.topology.reserve(3 * n)
    }
}

impl<P, ID, IC> IsVertexEditableMesh<P, Face3> for Mesh3D<P, ID, IC>
where
    P: IsEditable3D + IsBuildable3D + Clone,
    ID: IsDataContainer<P>,
    IC: IsIndexContainer,
{
    fn add_vertex(&mut self, vertex: P) -> VId {
        self.pc.push_d(vertex);
        VId {
            val: self.pc.len_d() - 1,
        }
    }

    fn change_vertex(&mut self, vid: VId, vertex: P) -> Result<()> {
        if vid.val < self.pc.len_d() {
            self.pc.set_d(vid.val, vertex);
            Ok(())
        } else {
            Err(ErrorKind::IncorrectVertexID)
        }
    }

    fn reserve_vertices(&mut self, n: usize) {
        self.pc.reserve_d(n)
    }
}

impl<P, ID, IC> HasBoundingBox3DMaybe for Mesh3D<P, ID, IC>
where
    P: Is3D,
    ID: IsDataContainer<P> + HasBoundingBox3DMaybe,
    IC: IsIndexContainer,
{
    fn bounding_box_maybe(&self) -> Option<BoundingBox3D> {
        self.pc.bounding_box_maybe()
    }
}

impl<P, ID, IC> HasCenterOfGravity3D for Mesh3D<P, ID, IC>
where
    P: Is3D,
    ID: IsDataContainer<P> + HasCenterOfGravity3D,
    IC: IsIndexContainer,
{
    fn center_of_gravity(&self) -> Option<Point3D> {
        self.pc.center_of_gravity()
    }
}

impl<P, ID, IC> IsScalable for Mesh3D<P, ID, IC>
where
    P: IsEditable3D,
    ID: IsDataContainer<P> + IsScalable,
    IC: IsIndexContainer,
{
    fn scale(&mut self, factor: Positive) {
        self.pc.scale(factor);
    }
}

impl<P, ID, IC> IsMatrix4Transformable for Mesh3D<P, ID, IC>
where
    P: Is3D + IsMatrix4Transformable + Clone,
    ID: IsDataContainer<P> + IsMatrix4Transformable + Clone,
    IC: IsIndexContainer,
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

impl<P, ID, IC> IsMovable3D for Mesh3D<P, ID, IC>
where
    P: Is3D + IsMovable3D,
    ID: IsDataContainer<P> + IsMovable3D,
    IC: IsIndexContainer,
{
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.pc.move_by(x, y, z)
    }
}

impl<P, ID, IC> From<(ID, IC)> for Mesh3D<P, ID, IC>
where
    P: Is3D,
    ID: IsDataContainer<P>,
    IC: IsIndexContainer,
{
    fn from(pt: (ID, IC)) -> Self {
        Self {
            pc: pt.0,
            topology: pt.1,
            _phantom: PhantomData::default(),
        }
    }
}

impl<P, ID, IC> Into<(ID, IC)> for Mesh3D<P, ID, IC>
where
    P: Is3D,
    ID: IsDataContainer<P>,
    IC: IsIndexContainer,
{
    fn into(self) -> (ID, IC) {
        (self.pc, self.topology)
    }
}

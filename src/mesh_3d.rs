use traits::is_mesh_3d::IsMesh3D;
use traits::has_editable_position_3d::HasEditablePosition3D;
use point_cloud_3d::PointCloud3D;

pub struct Mesh3D<P> where
    P: HasEditablePosition3D {

    pub pc: PointCloud3D<P>,
    pub topology: Vec<usize>
}

impl<P> IsMesh3D<P> for Mesh3D<P> where
    P: HasEditablePosition3D {

        fn num_faces(&self) -> usize {
            self.topology.len() / 3
        }

        fn num_vertices(&self) -> usize {
            self.pc.len()
        }

        fn face_vertex_ids(&self, faceid: usize) -> Option<(usize, usize, usize)> {
            let id1 = 3*faceid + 0;
            let id2 = 3*faceid + 1;
            let id3 = 3*faceid + 2;

            if id3 >= self.topology.len() {
                return None;
            }

            Some((id1, id2, id3))
        }

        fn face_vertices(&self, faceid: usize) -> Option<(P, P, P)> {
            match self.face_vertex_ids(faceid) {
                None => None,
                Some((id1, id2, id3)) => {
                    if id1 >= self.pc.len() || id2 >= self.pc.len() || id3 >= self.pc.len() {
                        return None;
                    }
                    if let (Some(v1), Some(v2), Some(v3)) = (self.vertex(id1), self.vertex(id2), self.vertex(id3)) {
                        return Some((v1, v2, v3));
                    }
                    return None;
                }
            }
        }

        fn vertex(&self, vertexid: usize) -> Option<P> {
            if vertexid >= self.pc.len() {
                return None;
            }
            return Some(self.pc.data[vertexid].clone())
        }




    }

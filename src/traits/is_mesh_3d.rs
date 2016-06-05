use traits::is_3d::Is3D;

pub trait IsMesh3D<P> where
    P: Is3D {

    fn num_faces(&self) -> usize;

    fn num_vertices(&self) -> usize;

    fn face_vertex_ids(&self, faceid: usize) -> Option<(usize, usize, usize)>;

    fn face_vertices(&self, faceid: usize) -> Option<(P, P, P)>;

    fn vertex(&self, vertexid: usize) -> Option<P>;
}

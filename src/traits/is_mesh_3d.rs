use traits::is_3d::Is3D;

pub trait IsMesh3D<P> where
    P: Is3D {

    fn num_faces() -> usize;

    fn num_vertices() -> usize;

    fn face_vertex_ids(faceid: usize) -> Option<(usize, usize, usize)>;

    fn face_vertices(faceid: usize) -> Option<(P, P, P)>;

    fn vertex(vertexid: usize) -> Option<P>;
}

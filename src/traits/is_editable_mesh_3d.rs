use traits::is_3d::Is3D;
use traits::is_mesh_3d::IsMesh3D;

pub trait IsEditableMesh3D<P> : IsMesh3D<P> where
    P: Is3D {

    fn add_vertex(&mut self, vertex: P) -> usize;

    fn add_face(&mut self, v1: P, v2: P, v3: P) -> usize;

    fn try_add_connection(&mut self, vid1: usize, vid2: usize, vid3: usize) -> Option<usize>;
}

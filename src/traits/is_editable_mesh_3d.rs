use traits::is_buildable_3d::IsBuildable3D;
use traits::is_mesh_3d::IsMesh3D;

pub trait IsEditableMesh3D<P> : IsMesh3D<P> where
    P: IsBuildable3D {

    fn new() -> Self;

    fn add_vertex(&mut self, vertex: P) -> usize;

    fn add_face(&mut self, v1: P, v2: P, v3: P) -> usize;

    fn try_add_connection(&mut self, vid1: usize, vid2: usize, vid3: usize) -> Option<usize>;
}

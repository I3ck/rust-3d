use traits::hasEditablePosition3D::HasEditablePosition3D;
use pointCloud3D::PointCloud3D;

pub trait IsTree3D<P> where P: HasEditablePosition3D {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn to_pointcloud(&self) -> PointCloud3D<P>;
    fn build(&mut self, pc : PointCloud3D<P>) -> bool;
}

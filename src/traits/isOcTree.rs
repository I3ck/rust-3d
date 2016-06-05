use traits::isTree3D::IsTree3D;
use traits::hasEditablePosition3D::HasEditablePosition3D;
use pointCloud3D::PointCloud3D;

pub trait IsOcTree<P> : IsTree3D<P> where P: HasEditablePosition3D {
    fn collect(&self, maxdepth: i8) -> PointCloud3D<P>;
}

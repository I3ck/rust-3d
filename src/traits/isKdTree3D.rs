use traits::isTree3D::IsTree3D;
use traits::hasEditablePosition3D::HasEditablePosition3D;
use pointCloud3D::PointCloud3D;

pub trait IsKdTree3D<P> : IsTree3D<P> where P: HasEditablePosition3D {
    fn nearest(&self, search: &P) -> Option<P>;
    fn knearest(&self, search: &P, n: usize) -> PointCloud3D<P>;
    fn in_sphere(&self, search: &P, radius: f64) -> PointCloud3D<P>;
    fn in_box(&self, search: &P, xSize: f64, ySize: f64, zSize: f64) -> PointCloud3D<P>;
}

use traits::is_editable_3d::IsEditable3D;
use point_cloud_3d::PointCloud3D;

pub trait IsTree3D<P> where
    P: IsEditable3D {

    fn new() -> Self;

    fn size(&self) -> usize;

    fn to_pointcloud(&self) -> PointCloud3D<P>;
    
    fn build(&mut self, pc : PointCloud3D<P>) -> bool;
}

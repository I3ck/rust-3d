use traits::has_editable_position_3d::HasEditablePosition3D;
use point_cloud_3d::PointCloud3D;

pub trait IsTree3D<P> where
    P: HasEditablePosition3D {

    fn new() -> Self;

    fn size(&self) -> usize;

    fn to_pointcloud(&self) -> PointCloud3D<P>;
    
    fn build(&mut self, pc : PointCloud3D<P>) -> bool;
}

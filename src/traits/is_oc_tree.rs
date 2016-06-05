use traits::is_tree_3d::IsTree3D;
use traits::has_editable_position_3d::HasEditablePosition3D;
use point_cloud_3d::PointCloud3D;

pub trait IsOcTree<P> : IsTree3D<P> where
    P: HasEditablePosition3D {
        
    fn collect(&self, maxdepth: i8) -> PointCloud3D<P>;
}

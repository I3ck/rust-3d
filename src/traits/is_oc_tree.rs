use traits::is_tree_3d::IsTree3D;
use traits::is_editable_3d::IsEditable3D;
use point_cloud_3d::PointCloud3D;

pub trait IsOcTree<P> : IsTree3D<P> where
    P: IsEditable3D {
        
    fn collect(&self, maxdepth: i8) -> PointCloud3D<P>;
}

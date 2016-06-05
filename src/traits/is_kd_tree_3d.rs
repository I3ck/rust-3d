use traits::is_tree_3d::IsTree3D;
use traits::has_editable_position_3d::HasEditablePosition3D;
use point_cloud_3d::PointCloud3D;

pub trait IsKdTree3D<P> : IsTree3D<P> where
    P: HasEditablePosition3D {

    fn nearest(&self, search: &P) -> Option<P>;

    fn knearest(&self, search: &P, n: usize) -> PointCloud3D<P>;

    fn in_sphere(&self, search: &P, radius: f64) -> PointCloud3D<P>;
    
    fn in_box(&self, search: &P, x_size: f64, y_size: f64, z_size: f64) -> PointCloud3D<P>;
}

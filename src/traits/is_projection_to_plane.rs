use traits::has_editable_position_2d::HasEditablePosition2D;
use traits::has_editable_position_3d::HasEditablePosition3D;
use traits::transformable_to_2d::TransFormableTo2D;
use traits::transformable_to_3d::TransFormableTo3D;
use traits::is_normalized_3d::IsNormalized3D;
use traits::is_plane_3d::IsPlane3D;
use point_cloud_2d::PointCloud2D;
use point_cloud_3d::PointCloud3D;


//@todo finish trait and add implementation
//@todo better method names
//@todo maybe implement projection methods within the pc
//@todo transformable traits required later on?
pub trait IsProjectionToPlane<P2,P3,N> where
    P2: HasEditablePosition2D + TransFormableTo3D,
    P3: HasEditablePosition3D + TransFormableTo2D,
    N: IsNormalized3D {

    fn from_2d<PL>(plane: PL, pc: PointCloud2D<P2>) -> Box<Self> where
        PL: IsPlane3D<P3,N>; //places 2d pc on plane, assuming plane 0/0 == pc2d 0/0

    fn from_3d<PL>(plane: PL, pc: PointCloud3D<P3>) -> Box<Self> where
        PL: IsPlane3D<P3,N>; //projects 3d pc onto plane from global coords

    fn projected_pointcloud_3d_global(&self) -> PointCloud3D<P3>;

    fn plane<PL>(&self) -> PL where
        PL: IsPlane3D<P3,N>;

    fn projected_pointcloud_2d_local(&self) -> PointCloud2D<P2>;
    
    //@todo add overload which lets one set the layer count?
    fn extrude(&self, distance: f64) -> (PointCloud3D<P3>, PointCloud3D<P3>); //@todo fst = on plane, snd within dist (maybe add data type for this)
}

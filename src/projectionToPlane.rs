use pointCloud2D::{PointCloud2D};
use plane3D::{Plane3D};

//use traits::{IsProjectionToPlane, IsPlane3D, HasPosition2D, HasPosition3D, TransFormableTo2D, TransFormableTo3D};
/*TODO
pub struct ProjectionToPlane<P2,P3,PL> where P2: HasPosition2D + TransFormableTo3D, P3: HasPosition3D + TransFormableTo2D, PL: IsPlane3D<P3> {
    pub pc: PointCloud2D<P2>,
    pub plane: PL
}

impl<P2,P3,PL> IsProjectionToPlane<P2,P3> for ProjectionToPlane<P2,P3,PL> where P2: HasPosition2D + TransFormableTo3D, P3: HasPosition3D + TransFormableTo2D, PL: IsPlane3D<P3> {
    fn from_2d<PL>(plane: PL, pc: PointCloud2D<P2>) -> Box<Self> where PL: IsPlane3D<P3> {
        Box::new(ProjectionToPlane{
            pc: pc,
            plane: plane
        })
    }

    fn from_3d<PL>(plane: PL, pc: PointCloud3D<P3>) -> Box<Self> where PL: IsPlane3D<P3> {
        let mut pc2d = PointCloud2D::<P2>::new();
        let mut plane = Plane3D::new();

        //@TODO NOT IMPLEMENTED
        //@TODO for each point within the 3d space, project it onto the plane
        //@TODO then transform these points into the local system


        Box::new(ProjectionToPlane{
            pc: pc2d,
            plane: plane
        })
    }


}
*/

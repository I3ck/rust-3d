/*
Copyright 2016 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//use point_cloud_2d::{PointCloud2D};
//use plane_3d::{Plane3D};

//use traits::{IsProjectionToPlane, IsPlane3D, IsBuildable2D, IsBuildable3D, TransFormableTo2D, TransFormableTo3D};
/*TODO
pub struct ProjectionToPlane<P2,P3,PL> where P2: IsBuildable2D + TransFormableTo3D, P3: IsBuildable3D + TransFormableTo2D, PL: IsPlane3D<P3> {
    pub pc: PointCloud2D<P2>,
    pub plane: PL
}

impl<P2,P3,PL> IsProjectionToPlane<P2,P3> for ProjectionToPlane<P2,P3,PL> where P2: IsBuildable2D + TransFormableTo3D, P3: IsBuildable3D + TransFormableTo2D, PL: IsPlane3D<P3> {
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

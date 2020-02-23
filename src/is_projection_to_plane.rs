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

//! IsProjectionToPlane trait used types which can be projected onto and from planes within 3D space

use crate::*;

//@todo finish trait and add implementation
//@todo better method names
//@todo maybe implement projection methods within the pc
//@todo transformable traits required later on?
/// IsProjectionToPlane is a trait used types which can be projected onto and from planes within 3D space
pub trait IsProjectionToPlane<P2, P3, N>: Sized
where
    P2: IsBuildable2D + IsTransFormableTo3D,
    P3: Is3D + IsTransFormableTo2D,
    N: IsNormalized3D,
{
    /// Should create a projection of the given 2d points on the given plane
    fn from_2d<PL>(plane: PL, pc: PointCloud2D<P2>) -> Self
    where
        PL: IsPlane3D<P3, N>; //places 2d pc on plane, assuming plane 0/0 == pc2d 0/0
    /// Should project the given 3d points onto the plane
    fn from_3d<PL>(plane: PL, pc: PointCloud3D<P3>) -> Self
    where
        PL: IsPlane3D<P3, N>; //projects 3d pc onto plane from global coords
    /// Should return the projected positions as 3d positions in the global coordinate system
    fn projected_pointcloud_3d_global(&self) -> PointCloud3D<P3>;
    /// Should the plane which is used for the projection
    fn plane<PL>(&self) -> PL
    where
        PL: IsPlane3D<P3, N>;
    /// Should return the projected positions as 2d positions in the plane's coordinate system
    fn projected_pointcloud_2d_local(&self) -> PointCloud2D<P2>;
    //@todo add overload which lets one set the layer count?
    /// Should extrude the projection into 3d space as seperate layers
    fn extrude(&self, distance: f64) -> (PointCloud3D<P3>, PointCloud3D<P3>); //@todo fst = on plane, snd within dist (maybe add data type for this)
}

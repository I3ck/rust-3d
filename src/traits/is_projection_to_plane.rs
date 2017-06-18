/*
Copyright 2016 Martin Buck
This file is part of rust-3d.
rust-3d is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rust-3d is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with rust-3d.  If not, see <http://www.gnu.org/licenses/>.
*/

//! IsProjectionToPlane trait used types which can be projected onto and from planes within 3D space

use traits::is_2d::*;
use traits::is_buildable_2d::*;
use traits::is_3d::*;
use traits::is_transformable_to_2d::*;
use traits::is_transformable_to_3d::*;
use traits::is_normalized_3d::*;
use traits::is_plane_3d::*;
use point_cloud_2d::*;
use point_cloud_3d::*;


//@todo finish trait and add implementation
//@todo better method names
//@todo maybe implement projection methods within the pc
//@todo transformable traits required later on?
/// IsProjectionToPlane is a trait used types which can be projected onto and from planes within 3D space
pub trait IsProjectionToPlane<P2,P3,N> where
    P2: Is2D + IsBuildable2D + IsTransFormableTo3D,
    P3: Is3D + IsTransFormableTo2D,
    N: IsNormalized3D {
    /// Should create a projection of the given 2d points on the given plane
    fn from_2d<PL>(plane: PL, pc: PointCloud2D<P2>) -> Box<Self> where
        PL: IsPlane3D<P3,N>; //places 2d pc on plane, assuming plane 0/0 == pc2d 0/0
    /// Should project the given 3d points onto the plane
    fn from_3d<PL>(plane: PL, pc: PointCloud3D<P3>) -> Box<Self> where
        PL: IsPlane3D<P3,N>; //projects 3d pc onto plane from global coords
    /// Should return the projected positions as 3d positions in the global coordinate system
    fn projected_pointcloud_3d_global(&self) -> PointCloud3D<P3>;
    /// Should the plane which is used for the projection
    fn plane<PL>(&self) -> PL where
        PL: IsPlane3D<P3,N>;
    /// Should return the projected positions as 2d positions in the plane's coordinate system
    fn projected_pointcloud_2d_local(&self) -> PointCloud2D<P2>;
    //@todo add overload which lets one set the layer count?
    /// Should extrude the projection into 3d space as seperate layers
    fn extrude(&self, distance: f64) -> (PointCloud3D<P3>, PointCloud3D<P3>); //@todo fst = on plane, snd within dist (maybe add data type for this)
}

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

use traits::is_editable_2d::IsEditable2D;
use traits::is_buildable_2d::IsBuildable2D;
use traits::is_editable_3d::IsEditable3D;
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
    P2: IsEditable2D + IsBuildable2D + TransFormableTo3D,
    P3: IsEditable3D + TransFormableTo2D,
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

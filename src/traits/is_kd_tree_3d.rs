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

use traits::is_tree_3d::IsTree3D;
use traits::is_editable_3d::IsEditable3D;
use point_cloud_3d::PointCloud3D;

pub trait IsKdTree3D<P> : IsTree3D<P> where
    P: IsEditable3D {

    fn nearest(&self, search: &P) -> Option<P>;

    fn knearest(&self, search: &P, n: usize) -> PointCloud3D<P>;

    fn in_sphere(&self, search: &P, radius: f64) -> PointCloud3D<P>;

    fn in_box(&self, search: &P, x_size: f64, y_size: f64, z_size: f64) -> PointCloud3D<P>;
}

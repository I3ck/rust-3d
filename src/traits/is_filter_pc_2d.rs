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

//! IsFilterPC2D trait used for filters for 2D position collections

use traits::is_2d::*;
use point_cloud_2d::*;
use view::*;

/// IsFilterPC2D is a trait used for filters for 2D position collections
pub trait IsFilterPC2D<P> where
    P: Is2D {
    /// Should filter the passed points by setting the flags within the view
    fn filter(&self, pc: &PointCloud2D<P>, view: &mut View); //@todo could have optional search structures   also define traits for different search structs e.g. trait solely to search in_box_2d
}

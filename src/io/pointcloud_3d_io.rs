/*
Copyright 2017 Martin Buck
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

//! Module for IO operations on 2D point clouds

use result::*;
use traits::is_editable_3d::*;
use traits::is_buildable_3d::*;
use point_cloud_3d::*;

use std::io::prelude::*;
use std::fs::File;

//@todo better name for params
//@todo "where" less specific?
/// Saves a PointCloud3D as x y z coordinates with a specified delimiter between coordinates and positions. E.g. used to create the .xyz file format or .csv files
pub fn save_xyz<P>(pc: &PointCloud3D<P>, filepath: &str, delim_coord: &str, delim_pos: &str) -> Result<()> where
    P: IsEditable3D + IsBuildable3D {

    let mut f = try!(File::create(filepath).map_err(|e| e.to_error_kind()));
    for p in &pc.data {
        let buffer = p.x().to_string()  + delim_coord
                   + &p.y().to_string() + delim_coord
                   + &p.z().to_string()
                   + delim_pos;
        try!(f.write_all(buffer.as_bytes()).map_err(|e| e.to_error_kind()));
    }
    Ok(())
}

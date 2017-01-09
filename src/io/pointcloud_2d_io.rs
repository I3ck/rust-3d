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

use traits::is_editable_2d::*;
use traits::is_buildable_2d::*;
use point_cloud_2d::*;

use std::io::prelude::*;
use std::fs::File;

//@todo better name for params
//@todo "where" less specific?
pub fn save_xy<P>(pc: &PointCloud2D<P>, filepath: &str, delim_coord: &str, delim_pos: &str) -> bool where
    P: IsEditable2D + IsBuildable2D {

    let mut f = match File::create(filepath) {
        Err(_) => return false,
        Ok(f) => f
    };

    for p in &pc.data {
        let buffer = p.x().to_string()  + delim_coord
                   + &p.y().to_string()
                   + delim_pos;
        match f.write_all(buffer.as_bytes()) {
            Err(_) => return false,
            Ok(_) => {}
        }
    }
    true
}

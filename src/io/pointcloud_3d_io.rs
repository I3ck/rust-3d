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

extern crate core;

use result::*;
use traits::is_3d::*;
use traits::is_buildable_3d::*;
use point_cloud_3d::*;

use self::core::str::FromStr;
use std::io::prelude::*;
use std::fs::File;

//@todo better name for params
/// Saves a PointCloud3D as x y z coordinates with a specified delimiter between coordinates and positions. E.g. used to create the .xyz file format or .csv files
pub fn save_xyz<P>(pc: &PointCloud3D<P>, filepath: &str, delim_coord: &str, delim_pos: &str) -> Result<()> where
    P: Is3D {

    let mut f = File::create(filepath).map_err(|e| e.to_error_kind())?;
    for p in &pc.data {
        let buffer = p.x().to_string()  + delim_coord
                   + &p.y().to_string() + delim_coord
                   + &p.z().to_string()
                   + delim_pos;
        f.write_all(buffer.as_bytes()).map_err(|e| e.to_error_kind())?;
    }
    Ok(())
}

//@todo better name for params
/// Loads a PointCloud3D as x y z coordinates with a specified delimiter between coordinates and positions. E.g. used to load the .xyz file format or .csv files
pub fn load_xyz<P>(filepath: &str, delim_coord: &str, delim_pos: &str) -> Result<PointCloud3D<P>> where
    P: Is3D + IsBuildable3D {

    let mut f = File::open(filepath)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;
    let lines = content.split(delim_pos);

    let mut pc = PointCloud3D::<P>::new();
    for line in lines {
        if line == "" {
            continue;
        }
        ///@todo write util for this (change the buildable method to support custom delimiter)
        let split = line.split(delim_coord);
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            3 => {
                let x = f64::from_str(words[0]).map_err(|e| e.to_error_kind())?;
                let y = f64::from_str(words[1]).map_err(|e| e.to_error_kind())?;
                let z = f64::from_str(words[2]).map_err(|e| e.to_error_kind())?;
                pc.push(*P::build(x,y,z))
            },
            _ => return Err(ErrorKind::ParseError)
        }
    }
    Ok(pc)
}

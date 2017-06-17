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

//! Module for IO of the xy file format

extern crate core;

use result::*;
use traits::is_2d::*;
use traits::is_buildable_2d::*;
use traits::is_random_accessible::*;
use traits::is_random_insertible::*;

use self::core::str::FromStr;
use std::io::prelude::*;
use std::fs::File;

/// Saves an IsRandomAccessible<Is2D> as x y coordinates with a specified delimiter between coordinates and positions. E.g. used to create the .xy file format or .csv files
pub fn save_xy<RA, P>(ra: &RA, filepath: &str, delim_coord: &str, delim_pos: &str) -> Result<()> where
    RA: IsRandomAccessible<P>,
    P: Is2D {

    let mut f = File::create(filepath).map_err(|e| e.to_error_kind())?;
    let n = ra.len();
    for i in 0..n {
        let ref p = ra[i];
        let buffer = p.x().to_string()  + delim_coord
                   + &p.y().to_string()
                   + delim_pos;
        f.write_all(buffer.as_bytes()).map_err(|e| e.to_error_kind())?;
    }
    Ok(())
}

/// Loads a IsRandomInsertible<Is2D> as x y coordinates with a specified delimiter between coordinates and positions. E.g. used to load the .xy file format or .csv files
pub fn load_xy<RI, P>(ri: &mut RI, filepath: &str, delim_coord: &str, delim_pos: &str) -> Result<()> where
    RI: IsRandomInsertible<P>,
    P: Is2D + IsBuildable2D {

    let mut f = File::open(filepath)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;
    let lines = content.split(delim_pos);

    for line in lines {
        if line == "" {
            continue;
        }
        ///@todo write util for this (change the buildable method to support custom delimiter)
        let split = line.split(delim_coord);
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            2 => {
                let x = f64::from_str(words[0]).map_err(|e| e.to_error_kind())?;
                let y = f64::from_str(words[1]).map_err(|e| e.to_error_kind())?;
                ri.push(*P::build(x,y))
            },
            _ => return Err(ErrorKind::ParseError)
        }
    }
    Ok(())
}

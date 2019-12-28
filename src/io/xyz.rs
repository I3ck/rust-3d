/*
Copyright 2017 Martin Buck

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

//! Module for IO of the xyz file format

use crate::*;

use core::str::FromStr;
use std::io::{Read, Write};

/// Saves an IsRandomAccessible<Is3D> as x y z coordinates with a specified delimiter between coordinates and positions. E.g. used to create the .xyz file format or .csv files
pub fn save_xyz<RA, P, W>(write: &mut W, ra: &RA, delim_coord: &str, delim_pos: &str) -> Result<()>
where
    RA: IsRandomAccessible<P>,
    P: Is3D,
    W: Write,
{
    let n = ra.len();
    for i in 0..n {
        let ref p = ra[i];
        let buffer = p.x().to_string()
            + delim_coord
            + &p.y().to_string()
            + delim_coord
            + &p.z().to_string()
            + delim_pos;
        write
            .write_all(buffer.as_bytes())
            .map_err(|e| e.to_error_kind())?;
    }
    Ok(())
}

/// Loads a IsRandomInsertible<Is3D> as x y z coordinates with a specified delimiter between coordinates and positions. E.g. used to load the .xyz file format or .csv file
pub fn load_xyz<RI, P, R>(
    read: &mut R,
    ri: &mut RI,
    delim_coord: &str,
    delim_pos: &str,
) -> Result<()>
where
    RI: IsRandomInsertible<P>,
    P: Is3D + IsBuildable3D,
    R: Read,
{
    let mut content = String::new();
    read.read_to_string(&mut content)?;
    let lines = content.split(delim_pos);

    for line in lines {
        if line == "" {
            continue;
        }
        let split = line.split(delim_coord);
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            3 => {
                let x = f64::from_str(words[0]).map_err(|e| e.to_error_kind())?;
                let y = f64::from_str(words[1]).map_err(|e| e.to_error_kind())?;
                let z = f64::from_str(words[2]).map_err(|e| e.to_error_kind())?;
                ri.push(P::new(x, y, z))
            }
            _ => return Err(ErrorKind::ParseError),
        }
    }
    Ok(())
}

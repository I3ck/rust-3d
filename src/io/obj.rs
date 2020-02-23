/*
Copyright 2020 Martin Buck

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

//! Module for IO of the obj file format

use crate::*;

use core::str::FromStr;

use std::io::BufRead;

//@todo offer both point cloud and mesh loading? (applies to .ply as well?)
//@todo many valid files won't be read correctly currently

/// Loads an IsMesh3D from the .obj file format
pub fn load_obj_mesh<EM, P, R>(read: &mut R, mesh: &mut EM) -> ObjResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut line_buffer = String::new();
    let mut i_line = 0;

    loop {
        line_buffer.clear();
        let n_read = read.read_line(&mut line_buffer)?;
        if n_read == 0 {
            break;
        }
        let line = line_buffer.trim_end();
        i_line += 1;

        if line.starts_with("v ") {
            let mut words = to_words(&line);

            // skip "v"
            words.next().ok_or(ObjError::LineParse(i_line))?;

            let x = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(ObjError::LineParse(i_line))?;
            let y = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(ObjError::LineParse(i_line))?;
            let z = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(ObjError::LineParse(i_line))?;

            mesh.add_vertex(P::new(x, y, z));
        } else if line.starts_with("f ") {
            let mut words = to_words(&line);

            // skip "f"
            words.next().ok_or(ObjError::LineParse(i_line))?;

            let mut tmp = words.next().ok_or(ObjError::LineParse(i_line))?;
            let a = usize::from_str(until(tmp, "/")).map_err(|_| ObjError::LineParse(i_line))?;

            tmp = words.next().ok_or(ObjError::LineParse(i_line))?;
            let b = usize::from_str(until(tmp, "/")).map_err(|_| ObjError::LineParse(i_line))?;

            tmp = words.next().ok_or(ObjError::LineParse(i_line))?;
            let c = usize::from_str(until(tmp, "/")).map_err(|_| ObjError::LineParse(i_line))?;

            // obj indexing starts at 1
            mesh.try_add_connection(VId { val: a - 1 }, VId { val: b - 1 }, VId { val: c - 1 })
                .map_err(|_| ObjError::InvalidMeshIndices(i_line))?;
        }
    }

    Ok(())
}

/// Loads IsPushable<Is3D> from the .obj file format
pub fn load_obj_points<IP, P, R>(read: &mut R, ip: &mut IP) -> ObjResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let mut line_buffer = String::new();
    let mut i_line = 0;

    loop {
        line_buffer.clear();
        let n_read = read.read_line(&mut line_buffer)?;
        if n_read == 0 {
            break;
        }
        let line = line_buffer.trim_end();
        i_line += 1;

        if line.starts_with("v ") {
            let mut words = to_words(&line);

            // skip "v"
            words.next().ok_or(ObjError::LineParse(i_line))?;

            let x = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(ObjError::LineParse(i_line))?;
            let y = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(ObjError::LineParse(i_line))?;
            let z = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(ObjError::LineParse(i_line))?;

            ip.push(P::new(x, y, z));
        }
    }

    Ok(())
}

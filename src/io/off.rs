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

//! Module for IO of the off file format

use crate::*;

use core::str::FromStr;

use std::{
    fmt,
    io::{BufRead, Error as ioError},
};

//------------------------------------------------------------------------------

/// Loads an IsMesh3D from the off file format
pub fn load_off_mesh<EM, P, R>(read: &mut R, mesh: &mut EM) -> OffResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut line_buffer = String::new();
    let mut i_line = 0;

    let mut off_seen = false;
    let mut counts = None;

    loop {
        line_buffer.clear();
        let n_read = read.read_line(&mut line_buffer)?;
        if n_read == 0 {
            break;
        }
        let line = line_buffer.trim_end();
        i_line += 1;

        if !off_seen && line.starts_with("OFF") {
            off_seen = true;
            continue;
        }

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        if counts.is_none() {
            let mut words = to_words_skip_empty(&line);
            let n_vertices = words
                .next()
                .and_then(|word| usize::from_str(word).ok())
                .ok_or(OffError::LineParse(i_line))?;
            let n_faces = words
                .next()
                .and_then(|word| usize::from_str(word).ok())
                .ok_or(OffError::LineParse(i_line))?;

            mesh.reserve_vertices(n_vertices);
            mesh.reserve_faces(n_faces);

            counts = Some([n_vertices, n_faces]);
            continue;
        }

        // safe since checked above
        if mesh.num_vertices() < counts.unwrap()[0] {
            let mut words = to_words_skip_empty(&line);

            let x = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(OffError::LineParse(i_line))?;

            let y = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(OffError::LineParse(i_line))?;

            let z = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(OffError::LineParse(i_line))?;

            mesh.add_vertex(P::new(x, y, z));
        } else {
            let mut words = to_words_skip_empty(&line);

            let count_face = words.next().ok_or(OffError::LineParse(i_line))?;

            if count_face == "3" {
                let a = words
                    .next()
                    .and_then(|word| usize::from_str(word).ok())
                    .ok_or(OffError::LineParse(i_line))?;

                let b = words
                    .next()
                    .and_then(|word| usize::from_str(word).ok())
                    .ok_or(OffError::LineParse(i_line))?;

                let c = words
                    .next()
                    .and_then(|word| usize::from_str(word).ok())
                    .ok_or(OffError::LineParse(i_line))?;

                mesh.try_add_connection(VId { val: a }, VId { val: b }, VId { val: c })
                    .map_err(|_| OffError::InvalidMeshIndices(i_line))?;
            }
        }
    }

    Ok(())
}

/// Loads IsPushable<Is3D> from the .off file format
pub fn load_off_points<IP, P, R>(read: &mut R, ip: &mut IP) -> OffResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let mut line_buffer = String::new();
    let mut i_line = 0;

    let mut off_seen = false;
    let mut n_vertices = None;
    let mut n_added = 0;

    loop {
        line_buffer.clear();
        let n_read = read.read_line(&mut line_buffer)?;
        if n_read == 0 {
            break;
        }
        let line = line_buffer.trim_end();
        i_line += 1;

        if !off_seen && line.starts_with("OFF") {
            off_seen = true;
            continue;
        }

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        if n_vertices.is_none() {
            let mut words = to_words_skip_empty(&line);
            n_vertices = Some(
                words
                    .next()
                    .and_then(|word| usize::from_str(word).ok())
                    .ok_or(OffError::LineParse(i_line))?,
            );
            ip.reserve(n_vertices.unwrap());

            continue;
        }

        // safe since checked above
        if n_added < n_vertices.unwrap() {
            let mut words = to_words_skip_empty(&line);

            let x = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(OffError::LineParse(i_line))?;

            let y = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(OffError::LineParse(i_line))?;

            let z = words
                .next()
                .and_then(|word| f64::from_str(word).ok())
                .ok_or(OffError::LineParse(i_line))?;

            ip.push(P::new(x, y, z));
            n_added += 1;
        } else {
            break;
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Error type for .off file operations
pub enum OffError {
    AccessFile,
    InvalidMeshIndices(usize),
    LineParse(usize),
}

/// Result type for .off file operations
pub type OffResult<T> = std::result::Result<T, OffError>;

impl fmt::Debug for OffError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::InvalidMeshIndices(x) => {
                write!(f, "File contains invalid mesh indices on line {}", x)
            }
        }
    }
}

impl From<ioError> for OffError {
    fn from(_error: ioError) -> Self {
        OffError::AccessFile
    }
}

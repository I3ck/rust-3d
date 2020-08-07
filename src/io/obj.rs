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

use std::{
    fmt,
    io::{BufRead, Error as ioError},
};

use super::{types::*, utils::*};

//------------------------------------------------------------------------------

/// Loads an IsMesh3D from the .obj file format
pub fn load_obj_mesh<EM, P, R>(read: &mut R, mesh: &mut EM) -> ObjResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut line_buffer = Vec::new();
    let mut i_line = 0;

    while let Ok(line) = fetch_line(read, &mut line_buffer) {
        i_line += 1;

        if line.starts_with(b"v ") {
            let mut words = to_words_skip_empty(line);

            // skip "v"
            words.next().ok_or(ObjError::Vertex).line(i_line, line)?;

            let x = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::Vertex)
                .line(i_line, line)?;

            let y = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::Vertex)
                .line(i_line, line)?;

            let z = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::Vertex)
                .line(i_line, line)?;

            mesh.add_vertex(P::new(x, y, z));
        } else if line.starts_with(b"f ") {
            let mut words = to_words_skip_empty(line);

            // skip "f"
            words.next().ok_or(ObjError::Face).line(i_line, line)?;

            let mut tmp = words.next().ok_or(ObjError::Face).line(i_line, line)?;
            let a: usize = from_ascii(until_bytes(tmp, b'/'))
                .ok_or(ObjError::Face)
                .line(i_line, line)?;

            tmp = words.next().ok_or(ObjError::Face).line(i_line, line)?;
            let b: usize = from_ascii(until_bytes(tmp, b'/'))
                .ok_or(ObjError::Face)
                .line(i_line, line)?;

            tmp = words.next().ok_or(ObjError::Face).line(i_line, line)?;
            let c: usize = from_ascii(until_bytes(tmp, b'/'))
                .ok_or(ObjError::Face)
                .line(i_line, line)?;

            // obj indexing starts at 1
            mesh.try_add_connection(VId { val: a - 1 }, VId { val: b - 1 }, VId { val: c - 1 })
                .or(Err(ObjError::InvalidMeshIndices))
                .line(i_line, line)?;
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
    let mut line_buffer = Vec::new();
    let mut i_line = 0;

    while let Ok(line) = fetch_line(read, &mut line_buffer) {
        i_line += 1;

        if line.starts_with(b"v ") {
            let mut words = to_words_skip_empty(line);

            // skip "v"
            words.next().ok_or(ObjError::Vertex).line(i_line, line)?;

            let x = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::Vertex)
                .line(i_line, line)?;

            let y = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::Vertex)
                .line(i_line, line)?;

            let z = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(ObjError::Vertex)
                .line(i_line, line)?;

            ip.push(P::new(x, y, z));
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Error type for .obj file operations
pub enum ObjError {
    AccessFile,
    InvalidMeshIndices,
    Face,
    Vertex,
}

/// Result type for .obj file operations
pub type ObjResult<T> = IOResult<T, ObjError>;

impl fmt::Debug for ObjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::Face => write!(f, "Unable to parse face"),
            Self::Vertex => write!(f, "Unable to parse vertex"),
            Self::InvalidMeshIndices => write!(f, "File contains invalid mesh indices"),
        }
    }
}

impl fmt::Display for ObjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ioError> for ObjError {
    fn from(_error: ioError) -> Self {
        ObjError::AccessFile
    }
}

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

use std::{io::BufRead, iter::FusedIterator, marker::PhantomData};

use super::{types::*, utils::*};

//------------------------------------------------------------------------------

/// Iterator to incrementally load points from a .obj file
pub struct ObjPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    read: R,
    is_done: bool,
    i_line: usize,
    line_buffer: Vec<u8>,
    phantom_p: PhantomData<P>,
}

impl<P, R> ObjPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    pub fn new(read: R) -> Self {
        Self {
            read,
            is_done: false,
            i_line: 0,
            line_buffer: Vec::new(),
            phantom_p: PhantomData,
        }
    }
}

impl<P, R> Iterator for ObjPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = IOResult<DataReserve<P>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        while let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
            self.i_line += 1;

            if line.starts_with(b"v ") {
                return Some(
                    fetch_vertex(self.i_line, line)
                        .map(|x| DataReserve::Data(x))
                        .map_err(|e| {
                            self.is_done = true;
                            e
                        }),
                );
            }
        }

        self.is_done = true;

        None
    }
}

impl<P, R> FusedIterator for ObjPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Iterator to incrementally load a mesh from a .obj file
pub struct ObjMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    read: R,
    is_done: bool,
    i_line: usize,
    line_buffer: Vec<u8>,
    phantom_p: PhantomData<P>,
}

impl<P, R> ObjMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    pub fn new(read: R) -> Self {
        Self {
            read,
            is_done: false,
            i_line: 0,
            line_buffer: Vec::new(),
            phantom_p: PhantomData,
        }
    }
}

impl<P, R> Iterator for ObjMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = IOResult<FaceDataReserve<P>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        while let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
            self.i_line += 1;

            if line.starts_with(b"v ") {
                return Some(
                    fetch_vertex(self.i_line, line)
                        .map(|x| FaceDataReserve::Data(x))
                        .map_err(|e| {
                            self.is_done = true;
                            e
                        }),
                );
            } else if line.starts_with(b"f ") {
                return Some(
                    fetch_face(self.i_line, line)
                        .map(|x| FaceDataReserve::Face(x))
                        .map_err(|e| {
                            self.is_done = true;
                            e
                        }),
                );
            }
        }

        self.is_done = true;

        None
    }
}

impl<P, R> FusedIterator for ObjMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Loads an IsMesh3D from the .obj file format
pub fn load_obj_mesh<EM, P, R>(read: R, mesh: &mut EM) -> IOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let iterator = ObjMeshIterator::new(read);

    for rd in iterator {
        match rd? {
            FaceDataReserve::Data(x) => {
                mesh.add_vertex(x);
            }
            FaceDataReserve::Face([a, b, c]) => {
                mesh.try_add_connection(VId(a), VId(b), VId(c))
                    .map_err(|_| IOError::InvalidMeshIndices)?;
            }
            io::types::FaceDataReserve::ReserveDataFaces(n_vertices, n_faces) => {
                mesh.reserve_vertices(n_vertices);
                mesh.reserve_faces(n_faces);
            }
            io::types::FaceDataReserve::ReserveDataFacesExact(n_vertices, n_faces) => {
                mesh.reserve_vertices_exact(n_vertices);
                mesh.reserve_faces_exact(n_faces);
            }
        }
    }

    Ok(())
}

/// Loads IsPushable<Is3D> from the .obj file format
pub fn load_obj_points<IP, P, R>(read: R, ip: &mut IP) -> IOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let iterator = ObjPointsIterator::new(read);

    for p in iterator {
        match p? {
            DataReserve::Data(x) => ip.push(x),
            DataReserve::Reserve(n) => ip.reserve(n),
            DataReserve::ReserveExact(n) => ip.reserve_exact(n),
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

#[inline(always)]
fn fetch_vertex<P>(i_line: usize, line: &[u8]) -> IOResult<P>
where
    P: IsBuildable3D,
{
    let mut words = to_words_skip_empty(line);

    // skip "v"
    words.next().ok_or(IOError::Vertex(Some(i_line)))?;

    let x = words
        .next()
        .and_then(|w| fast_float::parse(w).ok())
        .ok_or(IOError::Vertex(Some(i_line)))?;

    let y = words
        .next()
        .and_then(|w| fast_float::parse(w).ok())
        .ok_or(IOError::Vertex(Some(i_line)))?;

    let z = words
        .next()
        .and_then(|w| fast_float::parse(w).ok())
        .ok_or(IOError::Vertex(Some(i_line)))?;

    Ok(P::new(x, y, z))
}

#[inline(always)]
fn fetch_face(i_line: usize, line: &[u8]) -> IOResult<[usize; 3]> {
    let mut words = to_words_skip_empty(line);

    // skip "f"
    words.next().ok_or(IOError::Face(Some(i_line)))?;

    let mut tmp = words.next().ok_or(IOError::Face(Some(i_line)))?;
    let a: usize = from_ascii(until_bytes(tmp, b'/')).ok_or(IOError::Face(Some(i_line)))?;

    tmp = words.next().ok_or(IOError::Face(Some(i_line)))?;
    let b: usize = from_ascii(until_bytes(tmp, b'/')).ok_or(IOError::Face(Some(i_line)))?;

    tmp = words.next().ok_or(IOError::Face(Some(i_line)))?;
    let c: usize = from_ascii(until_bytes(tmp, b'/')).ok_or(IOError::Face(Some(i_line)))?;

    //@todo could fail if 0 in file
    //obj indexing starts at 1
    Ok([a - 1, b - 1, c - 1])
}

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
pub struct ObjPointsIterator<P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
    read: R,
    is_done: bool,
    i_line: usize,
    line_buffer: Vec<u8>,
    phantom_p: PhantomData<P>,
}

impl<P, R, const CHUNK_SIZE: usize> ObjPointsIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
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

impl<P, R, const CHUNK_SIZE: usize> Iterator for ObjPointsIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
    type Item = IOResult<StackVec<DataReserve<P>, CHUNK_SIZE>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let mut chunk = StackVec::default();

        loop {
            if chunk.is_full() {
                return Some(Ok(chunk));
            } else if let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
                self.i_line += 1;

                if line.starts_with(b"v ") {
                    match fetch_vertex(self.i_line, line) {
                        Err(e) => {
                            self.is_done = true;
                            return Some(Err(e));
                        }
                        Ok(x) => {
                            chunk.push(DataReserve::Data(x)).unwrap() // unwrap safe since we only call this if chunk.has_space()
                        }
                    }
                }
            } else {
                self.is_done = true;
                if chunk.has_data() {
                    return Some(Ok(chunk));
                }
                return None;
            }
        }
    }
}

impl<P, R, const CHUNK_SIZE: usize> FusedIterator for ObjPointsIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Iterator to incrementally load a mesh from a .obj file
pub struct ObjMeshIterator<P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
    read: R,
    is_done: bool,
    i_line: usize,
    line_buffer: Vec<u8>,
    phantom_p: PhantomData<P>,
}

impl<P, R, const CHUNK_SIZE: usize> ObjMeshIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
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

impl<P, R, const CHUNK_SIZE: usize> Iterator for ObjMeshIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
    type Item = IOResult<StackVec<FaceDataReserve<P>, CHUNK_SIZE>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let mut chunk = StackVec::default();

        loop {
            if chunk.is_full() {
                return Some(Ok(chunk));
            } else if let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
                self.i_line += 1;

                if line.starts_with(b"v ") {
                    match fetch_vertex(self.i_line, line) {
                        Err(e) => {
                            self.is_done = true;
                            return Some(Err(e));
                        }
                        Ok(x) => chunk.push(FaceDataReserve::Data(x)).unwrap(), // unwrap safe since we only call this if chunk.has_space()
                    }
                } else if line.starts_with(b"f ") {
                    match fetch_face(self.i_line, line) {
                        Err(e) => {
                            self.is_done = true;
                            return Some(Err(e));
                        }
                        Ok(x) => chunk.push(FaceDataReserve::Face(x)).unwrap(), // unwrap safe since we only call this if chunk.has_space()
                    }
                }
            } else {
                self.is_done = true;
                if chunk.has_data() {
                    return Some(Ok(chunk));
                }
                return None;
            }
        }
    }
}

impl<P, R, const CHUNK_SIZE: usize> FusedIterator for ObjMeshIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Loads an IsMesh3D from the .obj file format
pub fn load_obj_mesh<EM, P, R, const CHUNK_SIZE: usize>(read: R, mesh: &mut EM) -> IOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone + Default,
    R: BufRead,
{
    let iterator = ObjMeshIterator::<_, _, CHUNK_SIZE>::new(read);

    for chunk in iterator {
        for x in chunk? {
            match x {
                FaceDataReserve::Data(p) => {
                    mesh.add_vertex(p);
                }
                FaceDataReserve::Face([a, b, c]) => {
                    mesh.try_add_connection(VId(a), VId(b), VId(c))
                        .or(Err(IOError::InvalidMeshIndices))?;
                }
                FaceDataReserve::ReserveDataFaces(n_vertices, n_faces) => {
                    mesh.reserve_vertices(n_vertices);
                    mesh.reserve_faces(n_faces);
                }
                FaceDataReserve::ReserveDataFacesExact(n_vertices, n_faces) => {
                    mesh.reserve_vertices_exact(n_vertices);
                    mesh.reserve_faces_exact(n_faces);
                }
            }
        }
    }

    Ok(())
}

/// Loads IsPushable<Is3D> from the .obj file format
pub fn load_obj_points<IP, P, R, const CHUNK_SIZE: usize>(read: R, ip: &mut IP) -> IOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + Default,
    R: BufRead,
{
    let iterator = ObjPointsIterator::<_, _, CHUNK_SIZE>::new(read);

    for chunk in iterator {
        for x in chunk? {
            match x {
                DataReserve::Data(x) => ip.push(x),
                DataReserve::Reserve(x) => ip.reserve(x),
                DataReserve::ReserveExact(x) => ip.reserve_exact(x),
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

#[inline(always)]
fn fetch_vertex<P>(i_line: usize, line: &[u8]) -> IOResult<P>
where
    P: IsBuildable3D + Default,
{
    let mut words = to_words_skip_empty(line);

    // skip "v"
    words.next().ok_or(IOError::Vertex(Some(i_line)))?;

    let x = words
        .next()
        .and_then(|w| from_ascii(w))
        .ok_or(IOError::Vertex(Some(i_line)))?;

    let y = words
        .next()
        .and_then(|w| from_ascii(w))
        .ok_or(IOError::Vertex(Some(i_line)))?;

    let z = words
        .next()
        .and_then(|w| from_ascii(w))
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

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

use std::{io::BufRead, iter::FusedIterator, marker::PhantomData};

use super::{types::*, utils::*};

//------------------------------------------------------------------------------

/// Iterator to incrementally load points from a .off file
pub struct OffPointsIterator<P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
    read: R,
    is_done: bool,
    i_line: usize,
    line_buffer: Vec<u8>,
    off_seen: bool,
    n_vertices: Option<usize>,
    n_added: usize,
    phantom_p: PhantomData<P>,
}

impl<P, R, const CHUNK_SIZE: usize> OffPointsIterator<P, R, CHUNK_SIZE>
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
            off_seen: false,
            n_vertices: None,
            n_added: 0,
            phantom_p: PhantomData,
        }
    }
}

impl<P, R, const CHUNK_SIZE: usize> Iterator for OffPointsIterator<P, R, CHUNK_SIZE>
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

                if !self.off_seen && line.starts_with(b"OFF") {
                    self.off_seen = true;
                    continue;
                } else if line.is_empty() || line.starts_with(b"#") {
                    continue;
                } else if self.n_vertices.is_none() {
                    let mut words = to_words_skip_empty(line);
                    match words
                        .next()
                        .and_then(|word| from_ascii(word))
                        .ok_or(IOError::VertexCount(Some(self.i_line)))
                    {
                        Ok(n) => {
                            self.n_vertices = Some(n);
                            chunk
                                .push(DataReserve::ReserveExact(self.n_vertices.unwrap()))
                                .unwrap() // unwrap safe since we only call this if chunk.has_space()
                        }
                        Err(e) => {
                            self.is_done = true;
                            return Some(Err(e));
                        }
                    }
                }
                // safe since checked above
                else if self.n_added < self.n_vertices.unwrap() {
                    self.n_added += 1;
                    match fetch_vertex(self.i_line, line) {
                        Err(e) => {
                            self.is_done = true;
                            return Some(Err(e));
                        }
                        Ok(x) => chunk.push(DataReserve::Data(x)).unwrap(), // unwrap safe since we only call this if chunk.has_space()
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

impl<P, R, const CHUNK_SIZE: usize> FusedIterator for OffPointsIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Iterator to incrementally load a mesh from a .off file
pub struct OffMeshIterator<P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
    read: R,
    is_done: bool,
    i_line: usize,
    line_buffer: Vec<u8>,
    off_seen: bool,
    counts: Option<[usize; 2]>,
    n_vertices_added: usize,
    phantom_p: PhantomData<P>,
}

impl<P, R, const CHUNK_SIZE: usize> OffMeshIterator<P, R, CHUNK_SIZE>
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
            off_seen: false,
            counts: None,
            n_vertices_added: 0,
            phantom_p: PhantomData,
        }
    }

    #[inline(always)]
    fn fetch_face(i_line: usize, line: &[u8]) -> IOResult<[usize; 3]> {
        let mut words = to_words_skip_empty(line);

        let count_face = words.next().ok_or(IOError::FaceVertexCount)?;

        if count_face == b"3" {
            let a = words
                .next()
                .and_then(|word| from_ascii(word))
                .ok_or(IOError::Face(Some(i_line)))?;

            let b = words
                .next()
                .and_then(|word| from_ascii(word))
                .ok_or(IOError::Face(Some(i_line)))?;

            let c = words
                .next()
                .and_then(|word| from_ascii(word))
                .ok_or(IOError::Face(Some(i_line)))?;

            Ok([a, b, c])
        } else {
            Err(IOError::FaceVertexCount)
        }
    }

    #[inline(always)]
    fn fetch_counts(i_line: usize, line: &[u8]) -> IOResult<[usize; 2]> {
        let mut words = to_words_skip_empty(line);
        let n_vertices = words
            .next()
            .and_then(|word| from_ascii(word))
            .ok_or(IOError::VertexCount(Some(i_line)))?;
        let n_faces = words
            .next()
            .and_then(|word| from_ascii(word))
            .ok_or(IOError::FaceCount(Some(i_line)))?;

        Ok([n_vertices, n_faces])
    }
}

impl<P, R, const CHUNK_SIZE: usize> Iterator for OffMeshIterator<P, R, CHUNK_SIZE>
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

                if !self.off_seen && line.starts_with(b"OFF") {
                    self.off_seen = true;
                    continue;
                } else if line.is_empty() || line.starts_with(b"#") {
                    continue;
                } else if self.counts.is_none() {
                    match Self::fetch_counts(self.i_line, line) {
                        Ok(counts) => {
                            self.counts = Some(counts);
                            chunk
                                .push(FaceDataReserve::ReserveDataFaces(counts[0], counts[1]))
                                .unwrap() // unwrap safe since we only call this if chunk.has_space()
                        }
                        Err(e) => {
                            self.is_done = true;
                            return Some(Err(e));
                        }
                    }
                }
                // safe since checked above
                else if self.n_vertices_added < self.counts.unwrap()[0] {
                    self.n_vertices_added += 1;
                    match fetch_vertex(self.i_line, line) {
                        Err(e) => {
                            self.is_done = true;
                            return Some(Err(e));
                        }
                        Ok(x) => chunk.push(FaceDataReserve::Data(x)).unwrap(), // unwrap safe since we only call this if chunk.has_space()
                    }
                } else {
                    match Self::fetch_face(self.i_line, line) {
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

impl<P, R, const CHUNK_SIZE: usize> FusedIterator for OffMeshIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Loads an IsMesh3D from the off file format
pub fn load_off_mesh<EM, P, R, const CHUNK_SIZE: usize>(read: R, mesh: &mut EM) -> IOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone + Default,
    R: BufRead,
{
    let iterator = OffMeshIterator::<P, R, CHUNK_SIZE>::new(read);

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

/// Loads IsPushable<Is3D> from the .off file format
pub fn load_off_points<IP, P, R, const CHUNK_SIZE: usize>(read: R, ip: &mut IP) -> IOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + Default,
    R: BufRead,
{
    let iterator = OffPointsIterator::<_, _, CHUNK_SIZE>::new(read);

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
    P: IsBuildable3D,
{
    let mut words = to_words_skip_empty(line);

    let x = words
        .next()
        .and_then(|word| fast_float::parse(word).ok())
        .ok_or(IOError::Vertex(Some(i_line)))?;

    let y = words
        .next()
        .and_then(|word| fast_float::parse(word).ok())
        .ok_or(IOError::Vertex(Some(i_line)))?;

    let z = words
        .next()
        .and_then(|word| fast_float::parse(word).ok())
        .ok_or(IOError::Vertex(Some(i_line)))?;

    Ok(P::new(x, y, z))
}

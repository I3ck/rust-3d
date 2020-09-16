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
pub struct OffPointsIterator<P, R>
where
    P: IsBuildable3D,
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

impl<P, R> OffPointsIterator<P, R>
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
            off_seen: false,
            n_vertices: None,
            n_added: 0,
            phantom_p: PhantomData,
        }
    }
}

impl<P, R> Iterator for OffPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = IOResult2<DataReserve<P>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        while let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
            self.i_line += 1;

            if !self.off_seen && line.starts_with(b"OFF") {
                self.off_seen = true;
                continue;
            }

            if line.is_empty() || line.starts_with(b"#") {
                continue;
            }

            if self.n_vertices.is_none() {
                let mut words = to_words_skip_empty(line);
                match words
                    .next()
                    .and_then(|word| from_ascii(word))
                    .ok_or(IOError::VertexCount(Some(self.i_line)))
                    //@todo missing line info
                {
                    Ok(n) => {
                        self.n_vertices = Some(n);
                        return Some(Ok(DataReserve::Reserve(self.n_vertices.unwrap())));
                    }
                    Err(e) => {
                        self.is_done = true;
                        return Some(Err(e));
                    }
                }
            }

            // safe since checked above
            if self.n_added < self.n_vertices.unwrap() {
                self.n_added += 1;
                return Some(
                    fetch_vertex(self.i_line, line)
                        .map(|x| DataReserve::Data(x))
                        //@todo missing line info
                        .map_err(|e| {
                            self.is_done = true;
                            e
                        }),
                );
            } else {
                self.is_done = true;
                return None;
            }
        }

        self.is_done = true;

        None
    }
}

impl<P, R> FusedIterator for OffPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Iterator to incrementally load a mesh from a .off file
pub struct OffMeshIterator<P, R>
where
    P: IsBuildable3D,
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

impl<P, R> OffMeshIterator<P, R>
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
            off_seen: false,
            counts: None,
            n_vertices_added: 0,
            phantom_p: PhantomData,
        }
    }

    #[inline(always)]
    fn fetch_face(i_line: usize, line: &[u8]) -> IOResult2<[usize; 3]> {
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
    fn fetch_counts(i_line: usize, line: &[u8]) -> IOResult2<[usize; 2]> {
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

impl<P, R> Iterator for OffMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = IOResult2<FaceDataReserve<P>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        while let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
            self.i_line += 1;

            if !self.off_seen && line.starts_with(b"OFF") {
                self.off_seen = true;
                continue;
            }

            if line.is_empty() || line.starts_with(b"#") {
                continue;
            }

            if self.counts.is_none() {
                match Self::fetch_counts(self.i_line, line) {
                    Ok(counts) => {
                        self.counts = Some(counts);
                        return Some(Ok(FaceDataReserve::ReserveDataFaces(counts[0], counts[1])));
                    }
                    Err(e) => {
                        self.is_done = true;
                        return Some(Err(e));
                    }
                }
            }

            // safe since checked above
            return Some(
                (if self.n_vertices_added < self.counts.unwrap()[0] {
                    self.n_vertices_added += 1;

                    fetch_vertex(self.i_line, line).map(|x| FaceDataReserve::Data(x))
                } else {
                    Self::fetch_face(self.i_line, line)
                        .map(|x| FaceDataReserve::Face(x))
                        .map_err(|e| {
                            self.is_done = true;
                            e
                        })
                })
                .map_err(|e| {
                    self.is_done = true;
                    e
                }),
            );
        }

        self.is_done = true;

        None
    }
}

impl<P, R> FusedIterator for OffMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Loads an IsMesh3D from the off file format
pub fn load_off_mesh<EM, P, R>(read: R, mesh: &mut EM) -> IOResult2<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let iterator = OffMeshIterator::<P, R>::new(read);

    for rd in iterator {
        match rd? {
            FaceDataReserve::Face([a, b, c]) => {
                mesh.try_add_connection(VId(a), VId(b), VId(c))
                    .map_err(|_| IOError::InvalidMeshIndices)?;
            }
            FaceDataReserve::ReserveDataFaces(n_vertices, n_faces) => {
                mesh.reserve_vertices(n_vertices);
                mesh.reserve_faces(n_faces);
            }
            FaceDataReserve::Data(x) => {
                mesh.add_vertex(x);
            }
        }
    }

    Ok(())
}

/// Loads IsPushable<Is3D> from the .off file format
pub fn load_off_points<IP, P, R>(read: R, ip: &mut IP) -> IOResult2<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let iterator = OffPointsIterator::new(read);

    for rd in iterator {
        match rd? {
            DataReserve::Reserve(x) => ip.reserve(x),
            DataReserve::Data(x) => ip.push(x),
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

#[inline(always)]
fn fetch_vertex<P>(i_line: usize, line: &[u8]) -> IOResult2<P>
where
    P: IsBuildable3D,
{
    let mut words = to_words_skip_empty(line);

    let x = words
        .next()
        .and_then(|word| from_ascii(word))
        .ok_or(IOError::Vertex(Some(i_line)))?;

    let y = words
        .next()
        .and_then(|word| from_ascii(word))
        .ok_or(IOError::Vertex(Some(i_line)))?;

    let z = words
        .next()
        .and_then(|word| from_ascii(word))
        .ok_or(IOError::Vertex(Some(i_line)))?;

    Ok(P::new(x, y, z))
}
